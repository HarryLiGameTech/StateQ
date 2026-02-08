use crate::gate::DynamicTargetGate;
use crate::gate::elementary::ElementaryGate;
use crate::gate::unitary::{UnitaryDoubleGate, UnitaryDynamicGate, UnitarySingleGate};
use crate::{dispatch, into_variant};
use crate::algebra::{GateMat, Mat2, Mat4, ToMat, ToMat2, ToMat4};
use crate::operation::{DoubleTargetOperation, DynamicTargetOperation, ElementaryGateOperation, Operation, SingleTargetOperation, TargetDouble, TargetMultiple, TargetSingle};
use crate::operation::elementary::ElementaryOperation;
use crate::qubit::qubit_accessor::QubitAccessor;

#[derive(Clone, Debug)]
pub enum UnitaryOperation {
    Single(UnitarySingleOperation),
    Double(UnitaryDoubleOperation),
    Dynamic(UnitaryDynamicOperation),
}

impl ElementaryGateOperation for UnitaryOperation {
    fn get_gate(&self) -> ElementaryGate {
        use UnitaryOperation::*;
        dispatch!(self; Single | Double | Dynamic => |op| op.gate.clone().into())
    }

    fn get_target(&self) -> QubitAccessor {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct UnitarySingleOperation {
    gate: UnitarySingleGate,
    target: TargetSingle,
}

impl UnitarySingleOperation {
    pub fn new(gate: UnitarySingleGate, target: TargetSingle) -> Self {
        Self { gate, target }
    }

    pub fn from_mat(mat: Mat2, target: TargetSingle) -> Self {
        Self { gate: UnitarySingleGate(Box::new(mat)), target }
    }
}

impl ToMat2 for UnitarySingleOperation {
    fn to_mat2(&self) -> Mat2 {
        *self.gate.0
    }
}

impl SingleTargetOperation for UnitarySingleOperation {
    type Gate = UnitarySingleGate;

    fn get_gate(&self) -> Self::Gate {
        self.gate.clone()
    }

    fn get_target(&self) -> TargetSingle {
        self.target
    }
}

#[derive(Clone, Debug)]
pub struct UnitaryDoubleOperation {
    gate: UnitaryDoubleGate,
    target: TargetDouble,
}

impl UnitaryDoubleOperation {
    pub fn new(gate: UnitaryDoubleGate, target: TargetDouble) -> Self {
        Self { gate, target }
    }

    pub fn from_mat(mat: Mat4, target: TargetDouble) -> Self {
        Self { gate: UnitaryDoubleGate(Box::new(mat)), target }
    }
}

impl ToMat4 for UnitaryDoubleOperation {
    fn to_mat4(&self) -> Mat4 {
        *self.gate.0
    }
}

impl DoubleTargetOperation for UnitaryDoubleOperation {
    type Gate = UnitaryDoubleGate;

    fn get_gate(&self) -> Self::Gate {
        self.gate.clone()
    }

    fn get_target(&self) -> TargetDouble {
        self.target
    }
}

#[derive(Clone, Debug)]
pub struct UnitaryDynamicOperation {
    gate: UnitaryDynamicGate,
    target: TargetMultiple,
}

impl UnitaryDynamicOperation {
    pub fn new(gate: UnitaryDynamicGate, target: TargetMultiple) -> Self {
        Self { gate, target }
    }
}

impl ToMat for UnitaryDynamicOperation {
    fn to_mat(&self) -> GateMat {
        GateMat::dynamic(self.gate.mat().clone())
    }
}

impl DynamicTargetOperation for UnitaryDynamicOperation {
    type Gate = UnitaryDynamicGate;

    fn get_gate(&self) -> Self::Gate {
        self.gate.clone()
    }

    fn get_target(&self) -> TargetMultiple {
        self.target.clone()
    }
}

into_variant! {
    UnitarySingleOperation
        => UnitaryOperation::Single => ElementaryOperation::Unitary => Operation::Elementary;
    UnitaryDoubleOperation
        => UnitaryOperation::Double => ElementaryOperation::Unitary => Operation::Elementary;
    UnitaryDynamicOperation
        => UnitaryOperation::Dynamic => ElementaryOperation::Unitary => Operation::Elementary;
}
