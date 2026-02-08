pub mod unitary;
pub mod standard;
pub mod canonical;
pub mod custom;

use crate::{dispatch, into_variant, qubits};
use crate::gate::custom::CustomSingleGate;
use crate::gate::Dagger;
use crate::gate::elementary::{ElementaryGate, SingleGate};
use crate::algebra::{Mat2, ToMat, ToMat2};
use crate::operation::elementary::canonical::CanonicalOperation;
use crate::operation::elementary::custom::{CustomDoubleOperation, CustomOperation, CustomSingleOperation};
use crate::operation::elementary::standard::{StandardDoubleOperation, StandardOperation, StandardSingleOperation};
use crate::operation::{DoubleTargetOperation, DynamicTargetOperation, ElementaryGateOperation, Operation, SingleTargetOperation, TargetSingle};
use crate::operation::elementary::unitary::{UnitaryDoubleOperation, UnitaryOperation, UnitarySingleOperation};
use crate::qubit::qubit_accessor::QubitAccessor;

#[derive(Clone, Debug)]
pub enum ElementaryOperation {
    Standard(StandardOperation),
    Unitary(UnitaryOperation),
    Canonical(CanonicalOperation),
    Custom(CustomOperation),
}

impl ElementaryOperation {
    pub fn get_ident(&self) -> String {
        use ElementaryOperation::*;
        use crate::gate::DoubleTargetGate;
        dispatch!(self; Standard | Canonical | Custom => |op| op.get_gate().ident())
    }

    pub fn get_gate(&self) -> ElementaryGate {
        use ElementaryOperation::*;
        dispatch!(self; Standard | Canonical | Custom => |op| op.get_gate().clone().into())
    }
}

impl Dagger for ElementaryOperation {
    fn dagger(self) -> Self {
        use ElementaryOperation::*;
        dispatch!(self; Standard | Canonical | Custom => |op| op.dagger().into())
    }
}

impl ElementaryGateOperation for ElementaryOperation {
    fn get_gate(&self) -> ElementaryGate {
        use ElementaryOperation::*;
        dispatch!(self; Standard | Canonical | Custom => |op| op.get_gate().clone().into())
    }

    fn get_target(&self) -> QubitAccessor {
        match self {
            ElementaryOperation::Standard(op) => op.get_target(),
            ElementaryOperation::Unitary(op) => op.get_target(),
            ElementaryOperation::Custom(op) => op.get_target(),
            ElementaryOperation::Canonical(op) => {
                let (target0, target1) = op.get_target();
                qubits![target0, target1]
            },
        }
    }
}

into_variant! {
    ElementaryOperation => Operation::Elementary;
}

pub enum SingleOperation {
    Standard(StandardSingleOperation),
    Unitary(UnitarySingleOperation),
    Custom(CustomSingleOperation),
}

impl ToMat2 for SingleOperation {
    fn to_mat2(&self) -> Mat2 {
        use SingleOperation::*;
        dispatch!(self; Standard | Unitary | Custom => |op| op.to_mat2())
    }
}

impl Into<ElementaryOperation> for SingleOperation {
    fn into(self) -> ElementaryOperation {
        use SingleOperation::*;
        dispatch!(self; Standard | Unitary | Custom => |op| op.into())
    }
}

impl Into<Operation> for SingleOperation {
    fn into(self) -> Operation {
        use SingleOperation::*;
        dispatch!(self; Standard | Unitary | Custom => |op| op.into())
    }
}

impl SingleTargetOperation for SingleOperation {
    type Gate = SingleGate;

    fn get_gate(&self) -> Self::Gate {
        todo!()
    }

    fn get_target(&self) -> TargetSingle {
        use SingleOperation::*;
        dispatch!(self; Standard | Unitary | Custom => |op| op.get_target())
    }
}

impl TryFrom<ElementaryOperation> for SingleOperation {
    type Error = ();

    fn try_from(value: ElementaryOperation) -> Result<Self, Self::Error> {
        match value {
            ElementaryOperation::Standard(StandardOperation::Single(op)) => {
                Ok(SingleOperation::Standard(op))
            },
            ElementaryOperation::Unitary(UnitaryOperation::Single(op)) => {
                Ok(SingleOperation::Unitary(op))
            },
            ElementaryOperation::Custom(custom) => {
                match custom {
                    CustomOperation::Single(op) => Ok(SingleOperation::Custom(op)),
                    CustomOperation::Double(_) => Err(()),
                    CustomOperation::Dynamic(op) => {
                        let gate = op.get_gate();
                        if gate.size() == 1 {
                            let gate = CustomSingleGate::new(
                                gate.ident(),
                                gate.to_mat().try_into().unwrap(),
                                gate.get_params().clone()
                            );
                            let target = op.get_target();
                            Ok(SingleOperation::Custom(
                                CustomSingleOperation::new(gate, target[0])
                            ))
                        } else {
                            Err(())
                        }
                    }
                }
            },
            _ => Err(())
        }
    }
}

pub enum DoubleOperation {
    Standard(StandardDoubleOperation),
    Unitary(UnitaryDoubleOperation),
    Custom(CustomDoubleOperation),
}
