use nalgebra::{Const, DefaultAllocator, Dim, DimDiff, DimDiv, DimMax, DimMaximum, DimMin, DimSub, U1, U2, U3};
use nalgebra::allocator::Allocator;
use num::complex::{Complex64, ComplexFloat};
use crate::algebra::{DMat, IsHermitian, Mat};

pub struct Demultiplex<D: Dim>
where
    DefaultAllocator: Allocator<Complex64, D> + Allocator<Complex64, D, D>
{
    pub v_mat: Mat<D>,
    pub w_mat: Mat<D>,
    pub rz_angles: Vec<f64>,
}

impl<D: Dim + DimDiv<U2> + DimSub<U1>> Demultiplex<D>
where
    DefaultAllocator:
        Allocator<Complex64, D> +
        Allocator<Complex64, D, D> +
        Allocator<Complex64, <D as DimSub<U1>>::Output> +
        Allocator<Complex64, D, <D as DimSub<U1>>::Output>,
    DefaultAllocator:
        Allocator<f64, D> +
        Allocator<f64, <D as DimDiv<U2>>::Output> +
        Allocator<f64, <D as DimSub<U1>>::Output>,
{
    pub fn new(u1: &Mat<D>, u2: &Mat<D>) -> Self {
        assert_eq!(u1.shape(), u2.shape());
        let u1_u2h = u1 * u2.adjoint();
        let (eigen_vals, v_mat) = if u1_u2h.is_hermitian() {
            let eigen = u1_u2h.symmetric_eigen();
            (eigen.eigenvalues.map(Complex64::from), eigen.eigenvectors)
        } else {
            let (eigen_vals, v_mat) = u1_u2h.schur().unpack();
            (eigen_vals.diagonal(), v_mat)
        };
        // d_vals: complex vector
        let d_vals = eigen_vals.map(Complex64::sqrt);
        // d_mat: complex matrix
        let d_mat = Mat::<D>::from_partial_diagonal_generic(
            D::from_usize(d_vals.len()),
            D::from_usize(d_vals.len()),
            d_vals.as_slice()
        );
        // w_mat: complex matrix
        let w_mat = d_mat * v_mat.adjoint() * u2;
        // angles: f64 vector
        let angles = d_vals.conjugate().scale(2f64)
            .map(|z| f64::atan(z.im / z.re));
        Self {
            v_mat, w_mat,
            rz_angles: angles.as_slice().to_vec(),
        }
    }
}

pub struct DemultiplexRotation {

}

impl DemultiplexRotation {
    pub fn new(angles: &[f64]) -> DemultiplexRotation {
        let dim = angles.len();
        assert!(dim.is_power_of_two());
        todo!()
    }
}
