use nalgebra::{DefaultAllocator, Dim, DimDiv, DimSub, U1, U2};
use nalgebra::allocator::Allocator;
use num::complex::Complex64;
use crate::algebra::cos_sin::CosSinDecomposition;
use crate::algebra::demultiplex::Demultiplex;
use crate::algebra::Mat;

pub struct QuantumShannonDecomposition<D: Dim + DimDiv<U2>>
where
    DefaultAllocator:
    Allocator<Complex64, <D as DimDiv<U2>>::Output> +
    Allocator<Complex64, <D as DimDiv<U2>>::Output, <D as DimDiv<U2>>::Output>
{
    u1_mat: Mat<<D as DimDiv<U2>>::Output>,
    ucrz1_angles: Vec<f64>,
    u2_mat: Mat<<D as DimDiv<U2>>::Output>,
    ucry_angles: Vec<f64>,
    u3_mat: Mat<<D as DimDiv<U2>>::Output>,
    ucrz2_angles: Vec<f64>,
    u4_mat: Mat<<D as DimDiv<U2>>::Output>,
}

impl<D: Dim + DimDiv<U2> + DimSub<U1>> QuantumShannonDecomposition<D>
where
    DefaultAllocator:
        Allocator<Complex64, D> +
        Allocator<Complex64, D, D> +
        Allocator<Complex64, <D as DimDiv<U2>>::Output> +
        Allocator<Complex64, <D as DimDiv<U2>>::Output, <D as DimDiv<U2>>::Output> +
        Allocator<Complex64, <<D as DimDiv<U2>>::Output as DimSub<U1>>::Output> +
        Allocator<Complex64, <D as DimDiv<U2>>::Output, <<D as DimDiv<U2>>::Output as DimSub<U1>>::Output>,
    DefaultAllocator:
        Allocator<f64, D> +
        Allocator<f64, <D as DimDiv<U2>>::Output> +
        Allocator<f64, <<D as DimDiv<U2>>::Output as DimDiv<U2>>::Output> +
        Allocator<f64, <D as DimSub<U1>>::Output> +
        Allocator<f64, <<D as DimDiv<U2>>::Output as DimSub<U1>>::Output>,
    <D as DimDiv<U2>>::Output: DimDiv<U2>,
    <D as DimDiv<U2>>::Output: DimSub<U1>,
{
    pub fn new(mat: &Mat<D>) -> Self {
        assert_eq!(mat.ncols(), mat.nrows());
        let dim = mat.ncols();
        let size = dim.ilog2();
        assert!(size > 1);

        let CosSinDecomposition {
            u1, u2, cs, v1h, v2h
        } = CosSinDecomposition::<D>::new(mat);

        let u1_u2_demultiplexed = Demultiplex::<<D as DimDiv<U2>>::Output>::new(&u1, &u2);
        let u3_u4_demultiplexed = Demultiplex::<<D as DimDiv<U2>>::Output>::new(&v1h, &v2h);

        Self {
            u1_mat: u1_u2_demultiplexed.v_mat,
            ucrz1_angles: u1_u2_demultiplexed.rz_angles,
            u2_mat: u1_u2_demultiplexed.w_mat,
            ucry_angles: cs,
            u3_mat: u3_u4_demultiplexed.v_mat,
            ucrz2_angles: u3_u4_demultiplexed.rz_angles,
            u4_mat: u3_u4_demultiplexed.w_mat,
        }
    }
}
