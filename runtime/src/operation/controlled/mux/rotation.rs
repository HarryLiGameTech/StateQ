use crate::gate::{Dagger, DoubleTargetGate, SingleTargetGate};
use crate::gate::rotation::Rotation;
use crate::gate::standard::StandardDoubleGate::CX;
use crate::gate::standard::StandardSingleGate::X;
use crate::into_variant;
use crate::operation::controlled::ControlledOperation;
use crate::operation::controlled::mux::MuxOperation;
use crate::operation::controlled::single_ctrl::CtrlSingleTargetOperation;
use crate::operation::elementary::ElementaryOperation;
use crate::operation::Operation;
use crate::qubit::qubit_accessor::QubitAccessor;
use crate::qubit::QubitAddr;

#[derive(Clone, Debug)]
pub struct MuxRotationOperation {
    pub rotation: Rotation,
    pub ctrl: QubitAccessor,
    pub target: QubitAddr,
}

impl Dagger for MuxRotationOperation {
    fn dagger(self) -> Self {
        use Rotation::*;
        Self {
            ctrl: self.ctrl,
            target: self.target,
            rotation: match self.rotation {
                Rx(angle) => Rx(-angle),
                Ry(angle) => Ry(-angle),
                Rz(angle) => Rz(-angle),
            }
        }
    }
}

impl MuxRotationOperation {

    pub fn new(rotation: Rotation, ctrl: QubitAccessor, target: QubitAddr) -> Self {
        Self { rotation, ctrl, target }
    }

    pub fn decompose(&self) -> Vec<ElementaryOperation> {
        let mut operations = Vec::<ElementaryOperation>::new();
        if self.ctrl.size() == 1 {
            let ctrl = self.ctrl.first();
            // TODO: use CR gate instead
            let cr_gate_op = CtrlSingleTargetOperation::new(self.rotation, ctrl, self.target);
            let mut decomposed_cr = cr_gate_op.decompose();
            operations.push(X.apply_to(ctrl).into());
            operations.append(&mut decomposed_cr);
            operations.push(X.apply_to(ctrl).into());
            operations.append(&mut decomposed_cr);
        } else {
            let first_ctrl = self.ctrl.first();
            let mut new_ctrl = self.ctrl.clone();
            new_ctrl.remove_qubit(first_ctrl);
            let mut decompose_ucr = Self::new(self.rotation, new_ctrl, self.target).decompose();
            operations.append(&mut decompose_ucr);
            operations.push(CX.apply_to((first_ctrl, self.target)).into());
            operations.append(&mut decompose_ucr);
            operations.push(CX.apply_to((first_ctrl, self.target)).into());
        }
        operations
    }
}

into_variant! {
    MuxRotationOperation
        => MuxOperation::Rotation
        => ControlledOperation::Mux
        => Operation::Controlled;
}

