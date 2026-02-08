use crate::gate::canonical::CanonicalGate;
use crate::gate::{Dagger, DoubleTargetGate, DynamicTargetGate, IntoUnitary, SingleTargetGate};
use crate::gate::standard::{StandardGate, StandardSingleGate};
use crate::{dispatch, into_variant, algebra, use_enum};
use crate::gate::custom::{CustomGate, CustomSingleGate};
use crate::gate::unitary::{UnitaryDoubleGate, UnitaryGate, UnitarySingleGate};
use crate::algebra::{GateMat, Mat2, ToMat, ToMat2};
use crate::operation::elementary::{ElementaryOperation, SingleOperation};
use crate::operation::TargetSingle;
use crate::qubit::qubit_accessor::QubitAccessor;
use crate::algebra::close_to_zero;

/// Gates that can be easily represented by a unitary matrix.
#[derive(Clone, Debug)]
pub enum ElementaryGate {
    Standard(StandardGate),
    Unitary(UnitaryGate),
    Canonical(CanonicalGate),
    Custom(CustomGate),
}

into_variant! {
    StandardGate => ElementaryGate::Standard;
    UnitaryGate => ElementaryGate::Unitary;
    CanonicalGate => ElementaryGate::Canonical;
    CustomGate => ElementaryGate::Custom;
}

impl ElementaryGate {
    pub fn size(&self) -> usize {
        use ElementaryGate::*;
        match &self {
            Standard(gate) => gate.size(),
            Canonical(_) => 2usize,
            Custom(gate) => gate.size(),
            Unitary(gate) => todo!(),
        }
    }

    pub fn ident(&self) -> String {
        use ElementaryGate::*;
        dispatch!(self; Standard | Canonical | Custom => |gate| gate.ident())
    }

    pub fn apply_to(self, target: QubitAccessor) -> ElementaryOperation {
        match self {
            ElementaryGate::Standard(gate) => gate.apply_to(target).into(),
            ElementaryGate::Canonical(gate) => gate.apply_to((target[0], target[1])).into(),
            ElementaryGate::Custom(gate) => gate.apply_to(target).into(),
            ElementaryGate::Unitary(gate) => todo!(),
        }
    }
}

impl ToMat for ElementaryGate {
    fn to_mat(&self) -> GateMat {
        use ElementaryGate::*;
        dispatch!(self; Standard | Canonical | Custom => |gate| gate.to_mat())
    }
}

impl Dagger for ElementaryGate {
    fn dagger(self) -> Self {
        use ElementaryGate::*;
        dispatch!(self; Standard | Canonical | Custom => |gate| gate.dagger().into())
    }
}

impl From<SingleGate> for ElementaryGate {
    fn from(gate: SingleGate) -> Self {
        use SingleGate::*;
        dispatch!(gate; Standard | Custom => |gate| gate.into())
    }
}

impl TryInto<SingleGate> for ElementaryGate {
    type Error = ();

    fn try_into(self) -> Result<SingleGate, Self::Error> {
        use ElementaryGate::*;
        match self {
            Standard(StandardGate::Single(gate)) => Ok(SingleGate::Standard(gate)),
            Unitary(UnitaryGate::Single(gate)) => Ok(SingleGate::Unitary(gate)),
            Custom(CustomGate::Single(gate)) => Ok(SingleGate::Custom(gate)),
            _ => Err(())
        }
    }
}

pub fn is_identity(gate: &(impl Into<ElementaryGate> + Clone)) -> bool {
    use_enum!(ElementaryGate, StandardGate, StandardSingleGate);
    match gate.clone().into() {
        Standard(Single(I)) => true,
        Standard(Single(RX { angle })) if close_to_zero(angle) => true,
        Standard(Single(RY { angle })) if close_to_zero(angle) => true,
        Standard(Single(RZ { angle })) if close_to_zero(angle) => true,
        Standard(Single(P { angle })) if close_to_zero(angle) => true,
        Unitary(UnitaryGate::Single(UnitarySingleGate(mat))) => {
            algebra::is_identity(&*mat)
        },
        Unitary(UnitaryGate::Double(UnitaryDoubleGate(mat))) => {
            algebra::is_identity(&*mat)
        },
        // TODO: Custom gates
        // TODO: Dynamic gates
        _ => false,
    }
}

/// An elementary gate that acts on a single qubit.
#[derive(Clone, Debug)]
pub enum SingleGate {
    Standard(StandardSingleGate),
    Unitary(UnitarySingleGate),
    Custom(CustomSingleGate),
}

impl IntoUnitary<UnitarySingleGate> for SingleGate {
    fn into_unitary(self) -> UnitarySingleGate {
        UnitarySingleGate(Box::new(self.to_mat2()))
    }
}

impl SingleTargetGate for SingleGate {
    type Operation = SingleOperation;

    fn ident(&self) -> String {
        todo!()
    }

    fn apply_to(self, target: TargetSingle) -> Self::Operation {
        todo!()
    }
}

impl Dagger for SingleGate {
    fn dagger(self) -> Self {
        use SingleGate::*;
        dispatch!(self; Standard | Custom => |gate| gate.dagger().into())
    }
}

impl ToMat2 for SingleGate {
    fn to_mat2(&self) -> Mat2 {
        use SingleGate::*;
        dispatch!(self; Standard | Unitary | Custom => |gate| gate.to_mat2())
    }
}

into_variant! {
    StandardSingleGate => SingleGate::Standard;
    CustomSingleGate => SingleGate::Custom;
}
