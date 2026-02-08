use std::f64::consts::PI;
use crate::{c64, mat4, impl_to_mat};
use crate::algebra::{Mat4, ToMat, ToMat4, GateMat};
use num::complex::Complex64;
use crate::gate::{Dagger, DoubleTargetGate, IntoUnitary};
use crate::gate::unitary::UnitaryDoubleGate;
use crate::operation::TargetDouble;
use crate::operation::elementary::canonical::CanonicalOperation;

#[derive(Clone, Debug)]
pub struct CanonicalGate {
    pub tx: f64,
    pub ty: f64,
    pub tz: f64,
}

impl CanonicalGate {
    pub fn new(tx: f64, ty: f64, tz: f64) -> Self {
        Self { tx, ty, tz }
    }
}

impl_to_mat!(CanonicalGate: 4);

/// exp(-iπ/2 * (tx X⊗X + ty Y⊗Y + tz Z⊗Z))
impl ToMat4 for CanonicalGate {
    fn to_mat4(&self) -> Mat4 {
        let xx: Mat4 = mat4! {
            0f64, 0f64, 0f64, 1f64;
            0f64, 0f64, 1f64, 0f64;
            0f64, 1f64, 0f64, 0f64;
            1f64, 0f64, 0f64, 0f64;
        };
        let yy: Mat4 = mat4! {
             0f64, 0f64, 0f64, -1f64;
             0f64, 0f64, 1f64,  0f64;
             0f64, 1f64, 0f64,  0f64;
            -1f64, 0f64, 0f64,  0f64;
        };
        let zz: Mat4 = mat4! {
            1f64,  0f64,  0f64, 0f64;
            0f64, -1f64,  0f64, 0f64;
            0f64,  0f64, -1f64, 0f64;
            0f64,  0f64,  0f64, 1f64;
        };
        let mat = xx * c64!(self.tx) + yy * c64!(self.ty) + zz * c64!(self.tz);
        (mat * -Complex64::i() * c64!(PI * 0.5)).exp()
    }
}

impl IntoUnitary<UnitaryDoubleGate> for CanonicalGate {
    fn into_unitary(self) -> UnitaryDoubleGate {
        UnitaryDoubleGate(Box::new(self.to_mat4()))
    }
}

impl DoubleTargetGate for CanonicalGate {

    type Operation = CanonicalOperation;

    fn ident(&self) -> String {
        String::from("CAN")
    }

    fn apply_to(self, target: TargetDouble) -> Self::Operation {
        Self::Operation::new(self, target)
    }
}

impl Dagger for CanonicalGate {
    fn dagger(self) -> Self {
        Self::new(-self.tx, -self.ty, -self.tz)
    }
}
