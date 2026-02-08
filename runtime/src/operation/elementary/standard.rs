use crate::gate::standard::{StandardDoubleGate, StandardSingleGate, StandardTripleGate};
use crate::algebra::{Mat2, Mat4, Mat8, ToMat2, ToMat4, ToMat8, ToMat, GateMat};
use crate::operation::{
    SingleTargetOperation, DoubleTargetOperation, TripleTargetOperation, MultiTargetOperation,
    TargetSingle, TargetDouble, TargetTriple, ElementaryGateOperation,
};

use crate::{unwrap, impl_multi_target_op, impl_to_mat, into_variant, unwrap_variant, dispatch, qubits};
use crate::gate::Dagger;
use crate::gate::elementary::ElementaryGate;
use crate::operation::elementary::ElementaryOperation;
use crate::operation::Operation;
use crate::qubit::qubit_accessor::QubitAccessor;

#[derive(Clone, Debug)]
pub enum StandardOperation {
    Single(StandardSingleOperation),
    Double(StandardDoubleOperation),
    Triple(StandardTripleOperation),
}

impl ElementaryGateOperation for StandardOperation {
    fn get_gate(&self) -> ElementaryGate {
        use StandardOperation::*;
        dispatch!(self; Single | Double | Triple => |op| op.get_gate().into())
    }

    fn get_target(&self) -> QubitAccessor {
        match self {
            StandardOperation::Single(op) => qubits![op.target],
            StandardOperation::Double(op) => qubits!(op.target.0, op.target.1),
            StandardOperation::Triple(op) => qubits!(op.target.0, op.target.1, op.target.2)
        }
    }
}

impl Dagger for StandardOperation {
    fn dagger(self) -> Self {
        use StandardOperation::*;
        dispatch!(self; Single | Double | Triple => |op| op.dagger().into())
    }
}

into_variant! {
    StandardOperation => ElementaryOperation::Standard => Operation::Elementary;
}

/// StandardSingleOperation
#[derive(Clone, Debug)]
pub struct StandardSingleOperation {
    gate: StandardSingleGate,
    target: TargetSingle,
}

impl StandardSingleOperation {
    pub fn new(gate: StandardSingleGate, target: TargetSingle) -> Self {
        Self { gate, target }
    }
}

impl_to_mat!(StandardSingleOperation: 2);

impl ToMat2 for StandardSingleOperation {
    fn to_mat2(&self) -> Mat2 {
        self.gate.to_mat2()
    }
}

impl SingleTargetOperation for StandardSingleOperation {
    type Gate = StandardSingleGate;

    fn get_gate(&self) -> Self::Gate {
        self.gate
    }

    fn get_target(&self) -> TargetSingle {
        self.target
    }
}

impl Dagger for StandardSingleOperation {
    fn dagger(self) -> Self {
        Self {
            gate: self.gate.dagger(),
            target: self.target,
        }
    }
}

/// StandardDoubleOperation
#[derive(Clone, Debug)]
pub struct StandardDoubleOperation {
    gate: StandardDoubleGate,
    target: TargetDouble,
}

impl StandardDoubleOperation {
    pub fn new(gate: StandardDoubleGate, target: TargetDouble) -> Self {
        Self { gate, target }
    }
}

impl_to_mat!(StandardDoubleOperation: 4);

impl ToMat4 for StandardDoubleOperation {
    fn to_mat4(&self) -> Mat4 {
        self.gate.to_mat4()
    }
}

impl_multi_target_op!(StandardDoubleOperation: double);

impl DoubleTargetOperation for StandardDoubleOperation {
    type Gate = StandardDoubleGate;

    fn get_gate(&self) -> Self::Gate {
        self.gate
    }

    fn get_target(&self) -> TargetDouble {
        self.target
    }
}

impl Dagger for StandardDoubleOperation {
    fn dagger(self) -> Self {
        Self {
            gate: self.gate.dagger(),
            target: self.target,
        }
    }
}

/// StandardTripleOperation
#[derive(Clone, Debug)]
pub struct StandardTripleOperation {
    gate: StandardTripleGate,
    target: TargetTriple,
}

impl StandardTripleOperation {
    pub fn new(gate: StandardTripleGate, target: TargetTriple) -> Self {
        Self { gate, target }
    }
}

impl_to_mat!(StandardTripleOperation: 8);

impl ToMat8 for StandardTripleOperation {
    fn to_mat8(&self) -> Mat8 {
        self.gate.to_mat8()
    }
}

impl_multi_target_op!(StandardTripleOperation: triple);

impl TripleTargetOperation for StandardTripleOperation {
    type Gate = StandardTripleGate;

    fn get_gate(&self) -> Self::Gate {
        self.gate
    }

    fn get_target(&self) -> TargetTriple {
        self.target
    }
}

impl Dagger for StandardTripleOperation{
    fn dagger(self) -> Self {
        Self {
            gate: self.gate.dagger(),
            target: self.target,
        }
    }
}

into_variant! {
    StandardSingleOperation => StandardOperation::Single => ElementaryOperation::Standard => Operation::Elementary;
    StandardDoubleOperation => StandardOperation::Double => ElementaryOperation::Standard => Operation::Elementary;
    StandardTripleOperation => StandardOperation::Triple => ElementaryOperation::Standard => Operation::Elementary;
}
