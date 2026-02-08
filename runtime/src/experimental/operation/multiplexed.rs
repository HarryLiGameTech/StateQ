use crate::algebra::demultiplex::Demultiplex;
use crate::algebra::{demultiplex, DirectAdd, DMat, ToDMat};
use crate::algebra::quantum_shannon::QuantumShannonDecomposition;
use crate::circuit::Circuit;
use crate::operation::{Decompose, Operation, SingleOperation, Targets};
use crate::qubit::qubit_set::QubitSet;
use crate::qubit::qubit_vec::QubitVec;
use crate::qubit::QubitAddr;

pub struct MultiplexedOperation<T: Operation> {
    gates: Vec<T>,
    ctrls: QubitSet,
}

impl<T: Operation> MultiplexedOperation<T> {
    fn direct_sum(left: &[T], right: &[T]) -> DMat {
        assert_eq!(left.len(), right.len());
        assert!(left.len().is_power_of_two());
        if left.len() == 1 {
            return left[0].to_dyn_mat().direct_add(&right[0].to_dyn_mat());
        }
        let mid = left.len() / 2;
        let left = left.split_at(mid);
        let right = right.split_at(mid);
        Self::direct_sum(&left.0, &left.1).direct_add(
            &Self::direct_sum(&right.0, &right.1)
        )
    }

    // fn decompose(u1: &DMat, u2: &DMat) -> Circuit<dyn SingleOperation> {
    //     let demultiplexed = Demultiplex::new(&u1, &u2);
    //     Self::decompose(&demultiplexed.v_mat, &demultiplexed.w_mat);
    //     todo!()
    // }

}

impl<T: Operation> Decompose<T> for MultiplexedOperation<T> {
    fn decompose(&self) -> Circuit<T> {
        assert_eq!(self.gates.len(), 2usize.pow(self.ctrls.size() as u32));
        let (left, right) = self.gates.split_at(self.gates.len() / 2);
        let mid = left.len() / 2;
        let left = left.split_at(mid);
        let right = right.split_at(mid);
        let u1 = Self::direct_sum(left.0, left.1);
        let u2 = Self::direct_sum(right.0, right.1);
        Demultiplex::new(&u1, &u2);

        todo!()
    }

}

impl<OP: Operation> ToDMat for MultiplexedOperation<OP> {
    fn to_dyn_mat(&self) -> DMat {
        todo!()
    }
}

impl<OP: Operation> Targets for MultiplexedOperation<OP> {
    fn targets(&self) -> QubitVec {
        todo!()
    }
}

impl <OP: Operation> Operation for MultiplexedOperation<OP> {
    fn map_qubits(&self, f: &dyn Fn(QubitAddr) -> QubitAddr) -> Self where Self: Sized {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }
}
