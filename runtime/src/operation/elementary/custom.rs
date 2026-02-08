use crate::gate::custom::{CustomDoubleGate, CustomDynamicGate, CustomGate, CustomSingleGate};
use crate::gate::Dagger;
use crate::gate::elementary::ElementaryGate;
use crate::{dispatch, into_variant};
use crate::algebra::{GateMat, Mat2, Mat4, ToMat, ToMat2, ToMat4};
use crate::operation::{DoubleTargetOperation, DynamicTargetOperation, ElementaryGateOperation, MultiTargetOperation, SingleTargetOperation, TargetDouble, TargetMultiple, TargetSingle};
use crate::operation::elementary::ElementaryOperation;
use crate::qubit::qubit_accessor::QubitAccessor;
use crate::operation::Operation;

#[derive(Clone, Debug)]
pub enum CustomOperation {
    Single(CustomSingleOperation),
    Double(CustomDoubleOperation),
    Dynamic(CustomDynamicOperation),
}

impl ToMat for CustomOperation {
    fn to_mat(&self) -> GateMat {
        self.get_gate().to_mat()
    }
}

impl DynamicTargetOperation for CustomOperation {
    type Gate = CustomGate;

    fn get_gate(&self) -> Self::Gate {
        use CustomOperation::*;
        dispatch!(self; Single | Dynamic => |op| op.get_gate().clone().into())
    }

    fn get_target(&self) -> TargetMultiple {
        todo!()
    }
}

impl Dagger for CustomOperation {
    fn dagger(self) -> Self {
        use CustomOperation::*;
        dispatch!(self; Single | Dynamic => |op| op.dagger().into())
    }
}

into_variant! {
    CustomOperation => ElementaryOperation::Custom;
}

#[derive(Clone, Debug)]
pub struct CustomSingleOperation {
    gate: CustomSingleGate,
    target: TargetSingle,
}

impl CustomSingleOperation {
    pub fn new(gate: CustomSingleGate, target: TargetSingle) -> Self {
        Self { gate, target }
    }
}

impl ToMat2 for CustomSingleOperation {
    fn to_mat2(&self) -> Mat2 {
        self.gate.to_mat2()
    }
}

impl SingleTargetOperation for CustomSingleOperation {
    type Gate = CustomSingleGate;

    fn get_gate(&self) -> Self::Gate {
        self.gate.clone()
    }

    fn get_target(&self) -> TargetSingle {
        self.target
    }
}

impl Dagger for CustomSingleOperation {
    fn dagger(self) -> Self {
        Self { gate: self.gate.dagger(), ..self }
    }
}

#[derive(Clone, Debug)]
pub struct CustomDoubleOperation {
    gate: CustomDoubleGate,
    target: TargetDouble,
}

impl CustomDoubleOperation {
    pub fn new(gate: CustomDoubleGate, target: TargetDouble) -> Self {
        Self { gate, target }
    }
}

impl ToMat4 for CustomDoubleOperation {
    fn to_mat4(&self) -> Mat4 {
        self.gate.to_mat4()
    }
}

impl DoubleTargetOperation for CustomDoubleOperation {
    type Gate = CustomDoubleGate;

    fn get_gate(&self) -> Self::Gate {
        self.gate.clone()
    }

    fn get_target(&self) -> TargetDouble {
        self.target
    }
}

impl Dagger for CustomDoubleOperation {
    fn dagger(self) -> Self {
        Self { gate: self.gate.dagger(), ..self }
    }
}

#[derive(Clone, Debug)]
pub struct CustomDynamicOperation {
    gate: CustomDynamicGate,
    target: TargetMultiple,
}

impl CustomDynamicOperation {
    pub fn new(gate: CustomDynamicGate, target: TargetMultiple) -> Self {
        Self { gate, target }
    }
}

impl ToMat for CustomDynamicOperation {
    fn to_mat(&self) -> GateMat {
        self.gate.to_mat()
    }
}

impl MultiTargetOperation for CustomDynamicOperation {
    fn get_target_accessor(&self) -> QubitAccessor {
        self.target.clone()
    }
}

impl DynamicTargetOperation for CustomDynamicOperation {
    type Gate = CustomDynamicGate;

    fn get_gate(&self) -> Self::Gate {
        self.gate.clone()
    }

    fn get_target(&self) -> TargetMultiple {
        self.target.clone()
    }
}

impl Dagger for CustomDynamicOperation {
    fn dagger(self) -> Self {
        Self { gate: self.gate.dagger(), ..self }
    }
}

into_variant! {
    CustomSingleOperation
        => CustomOperation::Single
        => ElementaryOperation::Custom
        => Operation::Elementary;
    CustomDoubleOperation
        => CustomOperation::Double
        => ElementaryOperation::Custom
        => Operation::Elementary;
    CustomDynamicOperation
        => CustomOperation::Dynamic
        => ElementaryOperation::Custom
        => Operation::Elementary;
}
