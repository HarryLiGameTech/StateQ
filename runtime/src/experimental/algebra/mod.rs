
#[cfg(test)]
mod tests;
pub mod cos_sin;
pub mod zyz;
pub mod demultiplex;
pub mod quantum_shannon;
pub mod abc_decomp;
pub mod network_decomp;

use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use enum_dispatch::enum_dispatch;
use nalgebra::{ClosedAdd, ComplexField, Const, DefaultAllocator, Dim, DimDiff, DimMin, DimMul, DimName, DimNameAdd, DimNameSub, DimSub, DMatrix, Dynamic, Matrix, Matrix2, Matrix4, OMatrix, Scalar, SMatrix, Storage, ToTypenum, U1, U2, VecStorage};
use nalgebra::allocator::Allocator;
use num::complex::{Complex64, ComplexFloat};
use num::integer::Roots;
use num_traits::{abs, One, Pow, Zero};
use num_traits::real::Real;

pub type Mat2 = Matrix2<Complex64>;
pub type Mat4 = Matrix4<Complex64>;
pub type Mat8 = SMatrix<Complex64, 8, 8>;
pub type Mat<D> = OMatrix<Complex64, D, D>;
pub type DMat = DMatrix<Complex64>;
pub type CMat<const D: usize> = SMatrix<Complex64, D, D>;

pub const EPSILON: f64 = 1e-10;

pub trait ToMat2 {
    fn to_mat2(&self) -> Mat2;
}

pub trait ToMat4 {
    fn to_mat4(&self) -> Mat4;
}

pub trait ToMat8 {
    fn to_mat8(&self) -> Mat8;
}

#[enum_dispatch(QuantumOperation, ElementaryGate, StandardGate)]
pub trait ToDMat {
    fn to_dyn_mat(&self) -> DMat;
}

// impl<T: ToMat<Dynamic>> ToDMat for T {
//     fn to_dyn_mat(&self) -> DMat {
//         self.to_mat()
//     }
// }

pub trait ToMat<D: Dim>
where
    DefaultAllocator: Allocator<Complex64, D, D>
{
    fn to_mat(&self) -> Mat<D>;
}

#[macro_export]
macro_rules! impl_to_mat {
    (@inner $type:ty : $func:ident) => {
        impl ToMat for $type {
            fn to_mat(&self) -> GateMat {
                self.$func().into()
            }
        }
    };
    ($type:ty : 2) => { impl_to_mat!(@inner $type : to_mat2); };
    ($type:ty : 4) => { impl_to_mat!(@inner $type : to_mat4); };
    ($type:ty : 8) => { impl_to_mat!(@inner $type : to_mat8); };
    ($type:ty : dyn) => { impl_to_mat!(@inner $type : to_dyn_mat); };
}

trait IsClose {
    fn is_close(x: Self, y: Self) -> bool;
}

impl IsClose for f64 {
    fn is_close(x: Self, y: Self) -> bool {
        abs(x - y) < EPSILON
    }
}

impl IsClose for Complex64 {
    fn is_close(x: Self, y: Self) -> bool {
        f64::is_close(<Complex64 as ComplexField>::abs(x), <Complex64 as ComplexField>::abs(y))
    }
}

pub fn close_to_zero(x: f64) -> bool {
    x.abs() < EPSILON
}

/// Convert NxN unitary matrix to the special unitary group of degree N.
pub fn su<const RANK: usize>(mat: &CMat<RANK>) -> CMat<RANK>
where
    Const<RANK>: DimNameAdd<U1> + DimMin<Const<RANK>, Output = Const<RANK>>
{
    mat / mat.determinant().pow(1.0 / (RANK as f64))
}

/// Get the phase angle α in:
///  U = exp(iα) V
///  where V is an SU(2) matrix
pub fn phase_angle(mat: &Mat2) -> f64 {
    // α = arctan2(Im(det U), Re(det U)) / 2
    let det = mat.determinant();
    f64::atan2(det.im, det.re) / 2.0
}

pub trait MatSqrt {
    fn mat_sqrt(&self) -> Self;
}

impl<D: DimSub<U1> + DimMin<D, Output = D>> MatSqrt for Mat<D>
where
    Self: Clone + IndexMut<(usize, usize), Output = Complex64>,
    DefaultAllocator:
        Allocator<Complex64, D, DimDiff<D, U1>> +
        Allocator<Complex64, DimDiff<D, U1>> +
        Allocator<Complex64, D, D> +
        Allocator<Complex64, D>,
    DefaultAllocator: Allocator<bool, D, D>
{
    fn mat_sqrt(&self) -> Self {
        // A = Q U Q_{\dagger}, U is upper triangular
        let (q, u) = self.clone().schur().unpack();
        // B = U^{\frac{1}{2}}
        let mut b: Self = Self::zeros_generic(
            D::from_usize(self.nrows()),
            D::from_usize(self.ncols()),
        );
        let mut assigned = OMatrix::<bool, D, D>::from_element_generic(
            D::from_usize(self.nrows()),
            D::from_usize(self.ncols()),
            false
        );

        // B_{i, i} = \sqrt{U_{i, i}}
        for i in 0 .. self.nrows() {
            b[(i, i)] = u[(i, i)].sqrt();
            assigned[(i, i)] = true;
        }

        // To perform closure recursion, we need to wrap the environment
        struct CalcElemEnv<'s, D: DimSub<U1> + DimMin<D, Output = D>>
        where
            DefaultAllocator:
                Allocator<Complex64, D, DimDiff<D, U1>> +
                Allocator<Complex64, DimDiff<D, U1>> +
                Allocator<Complex64, D, D> +
                Allocator<Complex64, D>,
            DefaultAllocator: Allocator<bool, D, D>
        {
            pub assigned: &'s OMatrix<bool, D, D>,
            pub u: &'s OMatrix<Complex64, D, D>,
            pub b: &'s mut OMatrix<Complex64, D, D>,
        }

        // b_{i, j} = (u_{i, j} - \sum_{k=i+1}^{j-1} b_{i, k} b_{k, j}) / (u_{i, i} + u_{j, j})
        fn calc_b_elem<D>(env: &CalcElemEnv<D>, i: usize, j: usize) -> Complex64
        where
            D: DimSub<U1> + DimMin<D, Output = D>,
            DefaultAllocator:
                Allocator<Complex64, D, DimDiff<D, U1>> +
                Allocator<Complex64, DimDiff<D, U1>> +
                Allocator<Complex64, D, D> +
                Allocator<Complex64, D>,
            DefaultAllocator: Allocator<bool, D, D>
        {
            if env.assigned[(i, j)] {
                env.b[(i, j)]
            } else {
                (i + 1..j).fold(env.u[(i, j)], |acc, k| {
                    acc - calc_b_elem(env, i, k) * calc_b_elem(env, k, j)
                }) / (calc_b_elem(env, i, i) + calc_b_elem(env, j, j))
            }
        }

        // Iterate j - i from 1 to n - 1
        for diff in 1 .. self.nrows() {
            for i in 0 .. self.nrows() - diff {
                b[(i, i + diff)] = calc_b_elem(&CalcElemEnv {
                    assigned: &assigned,
                    u: &u,
                    b: &mut b
                }, i, i + diff);
                assigned[(i, i + diff)] = true;
            }
        }

        // A^{\frac{1}{2}} = Q U^{\frac{1}{2}} Q_{\dagger} = Q B Q_{\dagger}
        q.clone() * b * q.adjoint()
    }
}

pub trait MatEq<O> {
    fn mat_eq(&self, other: &O) -> bool;
}

impl<D1: Dim, D2: Dim> MatEq<Mat<D2>> for Mat<D1>
where
    DefaultAllocator: Allocator<Complex64, D1, D1>,
    DefaultAllocator: Allocator<Complex64, D2, D2>,
{
    fn mat_eq(&self, other: &Mat<D2>) -> bool {
        self.iter().zip(other.iter()).all(|(a, b)| {
            let diff = *a - *b;
            diff.re.abs() < EPSILON && diff.im.abs() < EPSILON
        })
    }
}

pub trait IsIdent {
    fn is_ident(&self) -> bool;
}

impl<D: Dim> IsIdent for Mat<D>
where
    DefaultAllocator: Allocator<Complex64, D, D>
{
    fn is_ident(&self) -> bool {
        assert!(self.is_square());
        let id = Self::identity_generic(
            D::from_usize(self.nrows()),
            D::from_usize(self.ncols()),
        );
        self.iter().zip(id.iter()).all(|(a, b)| {
            let diff = a - b;
            abs(diff.im) < EPSILON && abs(diff.re) < EPSILON
        })
    }
}

pub trait IsUnitary {
    fn is_unitary(&self) -> bool;
}

impl<D: Dim> IsUnitary for Mat<D>
where
    DefaultAllocator: Allocator<Complex64, D, D>
{
    fn is_unitary(&self) -> bool {
        (self * self.adjoint()).is_ident()
    }
}

pub trait IsHermitian {
    fn is_hermitian(&self) -> bool;
}

impl<D: Dim> IsHermitian for Mat<D>
where
    DefaultAllocator: Allocator<Complex64, D, D>
{
    fn is_hermitian(&self) -> bool {
        // self == &self.adjoint()
        self.mat_eq(&self.adjoint())
    }
}

pub trait DirectAdd<D: Dim + DimMul<U2>>
where
    DefaultAllocator: Allocator<Complex64, D, D>,
    DefaultAllocator: Allocator<
        Complex64, <D as DimMul<U2>>::Output, <D as DimMul<U2>>::Output
    >,
{
    fn direct_add(&self, other: &Mat<D>) -> Mat<<D as DimMul<U2>>::Output>;
}

impl<D: Dim + DimMul<U2>> DirectAdd<D> for Mat<D>
where
    DefaultAllocator: Allocator<Complex64, D, D>,
    DefaultAllocator: Allocator<
        Complex64, <D as DimMul<U2>>::Output, <D as DimMul<U2>>::Output
    >,
{
    fn direct_add(&self, other: &Mat<D>) -> Mat<<D as DimMul<U2>>::Output> {
        assert!(self.is_square());
        assert_eq!(self.nrows(), other.nrows());
        let dim = self.nrows();
        let mut mat = Mat::<<D as DimMul<U2>>::Output>::identity_generic(
            <D as DimMul<U2>>::Output::from_usize(dim * 2),
            <D as DimMul<U2>>::Output::from_usize(dim * 2),
        );
        for r in 0 .. self.nrows() {
            for c in 0 .. self.ncols() {
                mat[(r, c)] = self[(r, c)];
                mat[(r + dim, c + dim)] = other[(r, c)];
            }
        }
        mat
    }
}

#[macro_export]
macro_rules! c64 {
    ($im:literal i) => {
        {
            use num::complex::Complex64;
            Complex64::new(0f64, $im as f64)
        }
    };
    ($expr:expr) => {
        {
            use num::complex::Complex64;
            Complex64::new($expr as f64, 0f64)
        }
    };
}

#[macro_export]
macro_rules! mat2 {
    (
        $m11:expr, $m12:expr;
        $m21:expr, $m22:expr;
    ) => {
        {
            use num::complex::Complex64;
            Mat2::new(
                Complex64::from($m11), Complex64::from($m12),
                Complex64::from($m21), Complex64::from($m22),
            )
        }
    };
}

#[macro_export]
macro_rules! mat4 {
    (
        $m11:expr, $m12:expr, $m13:expr, $m14:expr;
        $m21:expr, $m22:expr, $m23:expr, $m24:expr;
        $m31:expr, $m32:expr, $m33:expr, $m34:expr;
        $m41:expr, $m42:expr, $m43:expr, $m44:expr;
    ) => {
        {
            use num::complex::Complex64;
            Mat4::new(
                Complex64::from($m11), Complex64::from($m12),
                Complex64::from($m13), Complex64::from($m14),
                Complex64::from($m21), Complex64::from($m22),
                Complex64::from($m23), Complex64::from($m24),
                Complex64::from($m31), Complex64::from($m32),
                Complex64::from($m33), Complex64::from($m34),
                Complex64::from($m41), Complex64::from($m42),
                Complex64::from($m43), Complex64::from($m44),
            )
        }
    };
}

#[macro_export]
macro_rules! const_mat4 {
    (
        $m11:expr, $m12:expr, $m13:expr, $m14:expr;
        $m21:expr, $m22:expr, $m23:expr, $m24:expr;
        $m31:expr, $m32:expr, $m33:expr, $m34:expr;
        $m41:expr, $m42:expr, $m43:expr, $m44:expr;
    ) => {
        {
            use num::complex::Complex64;
            Mat4::new(
                $m11, $m12, $m13, $m14,
                $m21, $m22, $m23, $m24,
                $m31, $m32, $m33, $m34,
                $m41, $m42, $m43, $m44,
            )
        }
    };
}
