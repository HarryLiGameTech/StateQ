use crate::gate::{Dagger, DoubleTargetGate, DynamicTargetGate, IntoUnitary, SingleTargetGate};
use crate::algebra::{GateMat, Mat2, Mat4, ToMat, ToMat2, ToMat4};
use crate::operation::elementary::custom::{CustomDoubleOperation, CustomDynamicOperation, CustomOperation, CustomSingleOperation};
use crate::operation::{TargetDouble, TargetMultiple, TargetSingle};
use crate::{dispatch, into_variant, raise_error};
use crate::gate::elementary::ElementaryGate;
use crate::gate::unitary::{UnitaryDoubleGate, UnitaryDynamicGate, UnitarySingleGate};

#[derive(Clone, Debug)]
pub enum CustomGate {
    Single(CustomSingleGate),
    Double(CustomDoubleGate),
    Dynamic(CustomDynamicGate),
}

impl CustomGate {
    pub fn size(&self) -> usize {
        match self {
            CustomGate::Single(_) => 1usize,
            CustomGate::Double(_) => 2usize,
            CustomGate::Dynamic(gate) => gate.size,
        }
    }

    pub fn ident(&self) -> String {
        use CustomGate::*;
        dispatch!(self; Single | Dynamic => |gate| gate.ident())
    }

    pub fn get_params(&self) -> &Vec<u64> {
        use CustomGate::*;
        dispatch!(self; Single | Dynamic => |gate| gate.get_params())
    }
}

impl ToMat for CustomGate {
    fn to_mat(&self) -> GateMat {
        match self {
            CustomGate::Single(gate) => gate.to_mat2().into(),
            CustomGate::Double(gate) => gate.to_mat4().into(),
            CustomGate::Dynamic(gate) => gate.to_mat(),
        }
    }
}

impl Dagger for CustomGate {
    fn dagger(self) -> Self {
        use CustomGate::*;
        dispatch!(self; Single | Dynamic => |gate| gate.dagger().into())
    }
}

impl IntoUnitary<UnitaryDynamicGate> for CustomGate {
    fn into_unitary(self) -> UnitaryDynamicGate {
        UnitaryDynamicGate::new(Box::new(self.to_mat().into()))
    }
}

impl DynamicTargetGate for CustomGate {
    type Operation = CustomOperation;
    fn apply_to(self, target: TargetMultiple) -> Self::Operation {
        match self {
            CustomGate::Single(gate) => {
                CustomSingleOperation::new(gate, target[0]).into()
            },
            CustomGate::Double(gate) => {
                CustomDoubleOperation::new(gate, (target[0], target[1])).into()
            },
            CustomGate::Dynamic(gate) => {
                CustomDynamicOperation::new(gate, target).into()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct CustomSingleGate {
    ident: String,
    matrix: Box<Mat2>,
    params: Vec<u64>,
}

impl CustomSingleGate {
    pub fn new(ident: String, matrix: Mat2, params: Vec<u64>) -> Self {
        Self { ident, matrix: Box::new(matrix), params }
    }

    pub fn get_params(&self) -> &Vec<u64> {
        &self.params
    }
}

impl ToMat2 for CustomSingleGate {
    fn to_mat2(&self) -> Mat2 {
        *self.matrix
    }
}

impl IntoUnitary<UnitarySingleGate> for CustomSingleGate {
    fn into_unitary(self) -> UnitarySingleGate {
        UnitarySingleGate(self.matrix)
    }
}

impl SingleTargetGate for CustomSingleGate {
    type Operation = CustomSingleOperation;

    fn ident(&self) -> String {
        self.ident.clone()
    }

    fn apply_to(self, target: TargetSingle) -> Self::Operation {
        Self::Operation::new(self, target)
    }
}

impl Dagger for CustomSingleGate {
    fn dagger(self) -> Self {
        Self {
            ident: format!("{}D", self.ident),
            matrix: Box::new(self.matrix.adjoint()),
            ..self
        }
    }
}

#[derive(Clone, Debug)]
pub struct CustomDoubleGate {
    ident: String,
    matrix: Box<Mat4>,
    params: Vec<u64>,
}

impl CustomDoubleGate {
    pub fn new(ident: String, matrix: Mat4, params: Vec<u64>) -> Self {
        Self { ident, matrix: Box::new(matrix), params }
    }

    pub fn get_params(&self) -> &Vec<u64> {
        &self.params
    }
}

impl ToMat4 for CustomDoubleGate {
    fn to_mat4(&self) -> Mat4 {
        *self.matrix
    }
}

impl IntoUnitary<UnitaryDoubleGate> for CustomDoubleGate {
    fn into_unitary(self) -> UnitaryDoubleGate {
        UnitaryDoubleGate(self.matrix)
    }
}

impl DoubleTargetGate for CustomDoubleGate {
    type Operation = CustomDoubleOperation;

    fn ident(&self) -> String {
        self.ident.clone()
    }

    fn apply_to(self, target: TargetDouble) -> Self::Operation {
        Self::Operation::new(self, target)
    }
}

impl Dagger for CustomDoubleGate {
    fn dagger(self) -> Self {
        Self {
            ident: format!("{}D", self.ident),
            matrix: Box::new(self.matrix.adjoint()),
            ..self
        }
    }
}

#[derive(Clone, Debug)]
pub struct CustomDynamicGate {
    ident: String,
    matrix: GateMat,
    size: usize,
    params: Vec<u64>,
}

impl CustomDynamicGate {
    pub fn new(ident: String, matrix: GateMat, size: usize, params: Vec<u64>) -> Self {
        assert!(size > 2);
        Self { ident, matrix, size, params }
    }

    pub fn ident(&self) -> String {
        self.ident.clone()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get_params(&self) -> &Vec<u64> {
        &self.params
    }
}

impl ToMat for CustomDynamicGate {
    fn to_mat(&self) -> GateMat {
        self.matrix.clone()
    }
}

impl IntoUnitary<UnitaryDynamicGate> for CustomDynamicGate {
    fn into_unitary(self) -> UnitaryDynamicGate {
        UnitaryDynamicGate::new(Box::new(self.matrix.into()))
    }
}

impl DynamicTargetGate for CustomDynamicGate {
    type Operation = CustomDynamicOperation;
    fn apply_to(self, target: TargetMultiple) -> Self::Operation {
        Self::Operation::new(self, target)
    }
}

impl Dagger for CustomDynamicGate {
    fn dagger(self) -> Self {
        Self {
            ident: format!("{}D", self.ident),
            matrix: self.matrix.dagger(),
            ..self
        }
    }
}

into_variant! {
    CustomSingleGate => CustomGate::Single => ElementaryGate::Custom;
    CustomDoubleGate => CustomGate::Double => ElementaryGate::Custom;
    CustomDynamicGate => CustomGate::Dynamic => ElementaryGate::Custom;
}
