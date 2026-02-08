use nalgebra::{DefaultAllocator, Dim, DimName, Dynamic};
use nalgebra::allocator::Allocator;
use num::complex::Complex64;
use crate::algebra::{DMat, Mat, ToDMat, ToMat};
use crate::operation::{Operation, StaticOperation, Targets};
use crate::qubit::qubit_vec::QubitVec;
use crate::qubit::QubitAddr;

pub type UnitaryGate = Unitary<Dynamic>;

pub struct Unitary<D: Dim>
where DefaultAllocator: Allocator<Complex64, D, D> {
    pub matrix: Mat<D>,
    pub qubits: Vec<QubitAddr>,
}

impl<D: Dim> Unitary<D>
where DefaultAllocator: Allocator<Complex64, D, D> {
    pub fn new(matrix: Mat<D>, qubits: impl Into<Vec<QubitAddr>>) -> Self {
        assert!(matrix.is_square());
        let qubits: Vec<QubitAddr> = qubits.into();
        assert_eq!(matrix.nrows(), 2usize.pow(qubits.len() as u32));
        Self { matrix, qubits }
    }
}

impl<D: Dim> ToDMat for Unitary<D>
where DefaultAllocator: Allocator<Complex64, D, D> {
    fn to_dyn_mat(&self) -> DMat {
        todo!()
    }
}

impl<D: Dim> Targets for Unitary<D>
where DefaultAllocator: Allocator<Complex64, D, D> {
    fn targets(&self) -> QubitVec {
        todo!()
    }
}

impl<D: Dim> Operation for Unitary<D>
where DefaultAllocator: Allocator<Complex64, D, D> {
    fn map_qubits(&self, f: &dyn Fn(QubitAddr) -> QubitAddr) -> Self where Self: Sized {
        todo!()
    }

    fn size(&self) -> usize {
        self.qubits.len()
    }
}

impl<D: DimName> ToMat<D> for Unitary<D>
where DefaultAllocator: Allocator<Complex64, D, D> {
    fn to_mat(&self) -> Mat<D> {
        self.matrix.clone()
    }
}

impl<D: DimName> StaticOperation<D> for Unitary<D>
where DefaultAllocator: Allocator<Complex64, D, D> {
    /* no functions */
}
