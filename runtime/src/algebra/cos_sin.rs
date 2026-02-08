use lapack::{c32, c64};
use nalgebra::{DefaultAllocator, Dim, DimDiv, U2};
use nalgebra::allocator::Allocator;
use num::complex::Complex64;
use num_traits::Zero;
use crate::algebra::{DMat, Mat};

pub struct CosSinDecomposition<D: Dim + DimDiv<U2>>
where
    DefaultAllocator:
        Allocator<Complex64, <D as DimDiv<U2>>::Output> +
        Allocator<Complex64, <D as DimDiv<U2>>::Output, <D as DimDiv<U2>>::Output>
{
    pub u1: Mat<<D as DimDiv<U2>>::Output>,
    pub u2: Mat<<D as DimDiv<U2>>::Output>,
    pub cs: Vec<f64>,
    pub v1h: Mat<<D as DimDiv<U2>>::Output>,
    pub v2h: Mat<<D as DimDiv<U2>>::Output>,
}

impl<D: Dim + DimDiv<U2>> CosSinDecomposition<D>
where
    DefaultAllocator:
        Allocator<Complex64, D> +
        Allocator<Complex64, D, D> +
        Allocator<Complex64, <D as DimDiv<U2>>::Output> +
        Allocator<Complex64, <D as DimDiv<U2>>::Output, <D as DimDiv<U2>>::Output>,
{
    pub fn new(mat: &Mat<D>) -> CosSinDecomposition<D> {
        assert_eq!(mat.nrows(), mat.ncols());
        let size = mat.nrows();
        let mut x11: Vec<c32> = mat.slice((0, 0), (size / 2, size / 2)).iter()
            .map(|x| c32::new(x.re as f32, x.im as f32)).collect::<Vec<c32>>();
        let mut x12 = mat.slice((0, size / 2), (size / 2, size / 2)).iter()
            .map(|x| c32::new(x.re as f32, x.im as f32)).collect::<Vec<c32>>();
        let mut x21 = mat.slice((size / 2, 0), (size / 2, size / 2)).iter()
            .map(|x| c32::new(x.re as f32, x.im as f32)).collect::<Vec<c32>>();
        let mut x22 = mat.slice((size / 2, size / 2), (size / 2, size / 2)).iter()
            .map(|x| c32::new(x.re as f32, x.im as f32)).collect::<Vec<c32>>();
        let mut angles = vec![0f32; size / 2];
        let mut u1 = vec![c32::zero(); 2usize.pow((size / 2) as u32)];
        let mut u2 = vec![c32::zero(); 2usize.pow((size / 2) as u32)];
        let mut v1h = vec![c32::zero(); 2usize.pow((size / 2) as u32)];
        let mut v2h = vec![c32::zero(); 2usize.pow((size / 2) as u32)];

        macro_rules! csd_call {
            { $csd:ident,
                $size:expr, $x11:expr, $x12:expr, $x21:expr, $x22:expr,
                $angles:expr, $u1:expr, $u2:expr, $v1h:expr, $v2h:expr
            } => {
                let mut work = vec![c32::new(0f32, 0f32)];
                let mut rwork = vec![0f32];
                let mut iwork = vec![0i32; size / 2];
                let mut result = 0i32;
                unsafe {
                    csd_call!(@call $csd,
                        $size, $x11, $x12, $x21, $x22,
                        $angles, $u1, $u2, $v1h, $v2h,
                        work.as_mut_slice(), -1,
                        rwork.as_mut_slice(), -1,
                        iwork.as_mut_slice(),
                        &mut result
                    );
                    assert_eq!(result, 0);
                    let lwork: i32 = std::mem::transmute_copy::<c32, i32>(&work[0]);
                    let lrwork: i32 = std::mem::transmute_copy::<f32, i32>(&rwork[0]);
                    let mut work = vec![c32::new(0f32, 0f32); lwork as usize];
                    let mut rwork = vec![0f32; lrwork as usize];
                    csd_call!(@call $csd,
                        $size, $x11, $x12, $x21, $x22,
                        $angles, $u1, $u2, $v1h, $v2h,
                        work.as_mut_slice(), lwork,
                        rwork.as_mut_slice(), lrwork,
                        iwork.as_mut_slice(),
                        &mut result
                    );
                    assert_eq!(result, 0);
                }
            };

            { @call $csd:ident,
                $size:expr, $x11:expr, $x12:expr, $x21:expr, $x22:expr,
                $angles:expr, $u1:expr, $u2:expr, $v1h:expr, $v2h:expr,
                $work:expr, $lwork:expr, $rwork:expr, $lrwork:expr,
                $iwork:expr, $result:expr
            } => {
                lapack::$csd(
                    'Y' as u8, // compute u1
                    'Y' as u8, // compute u2
                    'Y' as u8, // compute v1h
                    'Y' as u8, // compute v2h
                    'F' as u8, // trans
                    'F' as u8, // swap sign
                    $size as i32, // mat size
                    ($size / 2) as i32, // the number of rows in X11 and X12. 0 <= P <= M.
                    &[($size / 2)  as i32], // the number of cols in X11 and X21. 0 <= Q <= M.
                    $x11.as_mut_slice(), // x11 workspace
                    ($size / 2) as i32, // x11 size
                    $x12.as_mut_slice(), // x12 workspace
                    ($size / 2) as i32, // x12 size
                    $x21.as_mut_slice(), // x21 workspace
                    ($size / 2) as i32, // x21 size
                    $x22.as_mut_slice(), // x22 workspace
                    ($size / 2) as i32, // x22 size
                    $angles.as_mut_slice(),
                    $u1.as_mut_slice(),
                    ($size / 2) as i32,
                    $u2.as_mut_slice(),
                    ($size / 2) as i32,
                    $v1h.as_mut_slice(),
                    ($size / 2) as i32,
                    $v2h.as_mut_slice(),
                    ($size / 2) as i32,
                    $work, $lwork, $rwork, $lrwork, $iwork,
                    $result
                )
            };
        }

        csd_call! { cuncsd,
            size, x11, x12, x21, x22,
            angles, u1, u2, v1h, v2h
        }

        fn c32_slice_to_mat<D: Dim>(data: &[c32], size: usize) -> Mat<D>
        where
            DefaultAllocator: Allocator<Complex64, D> + Allocator<Complex64, D, D>
        {
            Mat::from_vec_generic(
                D::from_usize(size), D::from_usize(size),
                data.iter().map(|z| c64::new(z.re as f64, z.im as f64)).collect()
            )
        }

        Self {
            u1: c32_slice_to_mat::<<D as DimDiv<U2>>::Output>(&u1, size / 2),
            u2: c32_slice_to_mat::<<D as DimDiv<U2>>::Output>(&u2, size / 2),
            cs: angles.iter().map(|&x| x as f64).collect(),
            v1h: c32_slice_to_mat::<<D as DimDiv<U2>>::Output>(&v1h, size / 2),
            v2h: c32_slice_to_mat::<<D as DimDiv<U2>>::Output>(&v2h, size / 2),
        }
    }
}
