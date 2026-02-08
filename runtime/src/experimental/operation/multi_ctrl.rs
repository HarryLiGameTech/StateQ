use crate::algebra::{DMat, ToDMat};
use crate::circuit::Circuit;
use crate::operation::{Decompose, Operation, SingleOperation, Targets};
use crate::qubit::qubit_set::QubitSet;
use crate::qubit::qubit_vec::QubitVec;
use crate::qubit::QubitAddr;

pub struct MultiCtrlGate<OP: Operation> {
    operation: OP,
    ctrls: QubitSet,
}

impl<OP: Operation> MultiCtrlGate<OP> {
    pub fn new(operation: OP, ctrls: QubitSet) -> Self {
        Self { operation, ctrls }
    }
}

// impl<OP: Operation> Decompose<dyn SingleOperation> for MultiCtrlGate<OP> {
//     fn decompose(&self) -> Circuit<dyn SingleOperation> {
//         todo!()
//     }
// }

impl<OP: Operation> ToDMat for MultiCtrlGate<OP> {
    fn to_dyn_mat(&self) -> DMat {
        todo!()
    }
}

impl<OP: Operation> Targets for MultiCtrlGate<OP> {
    fn targets(&self) -> QubitVec {
        todo!()
    }
}

impl <OP: Operation> Operation for MultiCtrlGate<OP> {

    fn map_qubits(&self, f: &dyn Fn(QubitAddr) -> QubitAddr) -> Self where Self: Sized {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }
}
