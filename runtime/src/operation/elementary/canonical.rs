use crate::gate::canonical::CanonicalGate;
use crate::gate::standard::StandardDoubleGate::CX;
use crate::gate::standard::StandardSingleGate::{S, SD, YPOW, ZPOW};
use crate::{impl_multi_target_op, impl_to_mat, into_variant};
use crate::gate::{Dagger, DoubleTargetGate, SingleTargetGate};
use crate::algebra::{Mat4, ToMat4, ToMat, GateMat};
use crate::operation::{
    DoubleTargetOperation, MultiTargetOperation,
    TargetDouble,
};
use crate::operation::elementary::ElementaryOperation;
use crate::operation::Operation;
use crate::qubit::qubit_accessor::QubitAccessor;


#[derive(Clone, Debug)]
pub struct CanonicalOperation {
    gate: CanonicalGate,
    target: TargetDouble,
}

impl CanonicalOperation {
    pub fn new(gate: CanonicalGate, target: TargetDouble) -> Self {
        Self { gate, target }
    }

    fn cnot_decompose(&self) -> Vec<ElementaryOperation> {
        vec![
            S.apply_to(self.target.1).into(),
            CX.apply_to((self.target.1, self.target.0)).into(),
            ZPOW { t: self.gate.tz - 0.5 }.apply_to(self.target.0).into(),
            YPOW { t: self.gate.tx - 0.5 }.apply_to(self.target.1).into(),
            CX.apply_to((self.target.0, self.target.1)).into(),
            YPOW { t: 0.5 - self.gate.ty }.apply_to(self.target.1).into(),
            CX.apply_to((self.target.1, self.target.0)).into(),
            SD.apply_to(self.target.0).into(),
        ]
    }
}

impl_to_mat!(CanonicalOperation: 4);

impl ToMat4 for CanonicalOperation {
    fn to_mat4(&self) -> Mat4 {
        self.gate.to_mat4()
    }
}

impl_multi_target_op!(CanonicalOperation: double);

impl DoubleTargetOperation for CanonicalOperation {
    type Gate = CanonicalGate;

    fn get_gate(&self) -> Self::Gate {
        self.gate.clone()
    }

    fn get_target(&self) -> TargetDouble {
        self.target
    }
}

impl Dagger for CanonicalOperation {
    fn dagger(self) -> Self {
        Self {
            gate: self.gate.dagger(),
            target: self.target,
        }
    }
}

into_variant! {
    CanonicalOperation => ElementaryOperation::Canonical => Operation::Elementary;
}
