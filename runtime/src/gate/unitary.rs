use crate::gate::{DoubleTargetGate, DynamicTargetGate, IntoUnitary, SingleTargetGate};
use crate::gate::elementary::{ElementaryGate, SingleGate};
use crate::into_variant;
use crate::algebra::{DMat, Mat2, Mat4, ToMat2};
use crate::operation::elementary::unitary::{UnitaryDoubleOperation, UnitaryDynamicOperation, UnitarySingleOperation};
use crate::operation::{TargetDouble, TargetMultiple, TargetSingle};

#[derive(Clone, Debug)]
pub enum UnitaryGate {
    Single(UnitarySingleGate),
    Double(UnitaryDoubleGate),
    Dynamic(UnitaryDynamicGate),
}

/// Single-target unitary gate.
#[derive(Clone, Debug)]
pub struct UnitarySingleGate(pub Box<Mat2>);

pub const SINGLE_UNITARY_IDENT: &str = "_1";

impl IntoUnitary<UnitarySingleGate> for UnitarySingleGate {
    fn into_unitary(self) -> UnitarySingleGate { self }
}

impl SingleTargetGate for UnitarySingleGate {
    type Operation = UnitarySingleOperation;

    fn ident(&self) -> String {
        SINGLE_UNITARY_IDENT.to_string()
    }

    fn apply_to(self, target: TargetSingle) -> Self::Operation {
        Self::Operation::new(self, target)
    }
}

impl ToMat2 for UnitarySingleGate {
    fn to_mat2(&self) -> Mat2 {
        *self.0
    }
}

/// Double-target unitary gate.
#[derive(Clone, Debug)]
pub struct UnitaryDoubleGate(pub Box<Mat4>);

pub const DOUBLE_UNITARY_IDENT: &str = "_2";

impl IntoUnitary<UnitaryDoubleGate> for UnitaryDoubleGate {
    fn into_unitary(self) -> UnitaryDoubleGate { self }
}

impl DoubleTargetGate for UnitaryDoubleGate {
    type Operation = UnitaryDoubleOperation;

    fn ident(&self) -> String {
        DOUBLE_UNITARY_IDENT.to_string()
    }

    fn apply_to(self, target: TargetDouble) -> Self::Operation {
        Self::Operation::new(self, target)
    }
}

/// Dynamic-target unitary gate.
#[derive(Clone, Debug)]
pub struct UnitaryDynamicGate(Box<DMat>);

impl UnitaryDynamicGate {
    pub fn new(mat: Box<DMat>) -> Self {
        assert_eq!(mat.nrows(), mat.ncols());
        assert!(mat.nrows() > 4);
        assert!(mat.nrows().is_power_of_two());
        Self(mat)
    }

    pub fn mat(&self) -> &DMat {
        &self.0
    }
}

impl IntoUnitary<UnitaryDynamicGate> for UnitaryDynamicGate {
    fn into_unitary(self) -> UnitaryDynamicGate { self }
}

impl DynamicTargetGate for UnitaryDynamicGate {
    type Operation = UnitaryDynamicOperation;

    fn apply_to(self, target: TargetMultiple) -> Self::Operation {
        Self::Operation::new(self, target)
    }
}

into_variant! {
    UnitarySingleGate => SingleGate::Unitary;
    UnitarySingleGate => UnitaryGate::Single => ElementaryGate::Unitary;
    UnitaryDoubleGate => UnitaryGate::Double => ElementaryGate::Unitary;
    UnitaryDynamicGate => UnitaryGate::Dynamic => ElementaryGate::Unitary;
}
