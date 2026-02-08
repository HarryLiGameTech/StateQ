use gates_def::GatesDef;
use strum_macros::{EnumVariantNames, IntoStaticStr};
use crate::gate::elementary::ElementaryGate;
use crate::{dispatch, into_variant, impl_to_mat, unwrap, unwrap_variant};
use crate::gate::{Dagger, DoubleTargetGate, IntoUnitary, SingleTargetGate, TripleTargetGate};
use crate::gate::unitary::{UnitaryDoubleGate, UnitarySingleGate};
use crate::algebra::{GateMat, Mat2, Mat4, Mat8, ToMat, ToMat2, ToMat4, ToMat8};
use crate::operation::{TargetDouble, TargetSingle, TargetTriple};
use crate::operation::elementary::standard::{StandardSingleOperation, StandardDoubleOperation, StandardTripleOperation, StandardOperation};
use crate::qubit::qubit_accessor::QubitAccessor;
use crate::macros::Unwrap;

#[derive(Debug, Copy, Clone)]
pub enum StandardGate {
    Single(StandardSingleGate),
    Double(StandardDoubleGate),
    Triple(StandardTripleGate),
}

impl StandardGate {
    pub fn apply_to(&self, target: QubitAccessor) -> StandardOperation {
        match self {
            StandardGate::Single(gate) => gate.apply_to(target[0]).into(),
            StandardGate::Double(gate) => gate.apply_to((target[0], target[1])).into(),
            StandardGate::Triple(gate) => gate.apply_to((target[0], target[1], target[2])).into(),
        }
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq,
    GatesDef, IntoStaticStr, EnumVariantNames,
)]
pub enum StandardSingleGate {

    /// Identity
    #[mat(
    | 1, 0 |
    | 0, 1 |
    )]
    #[dagger(I)]
    I,

    /// Hadamard
    #[mat(
    | 1 / sqrt(2),  1 / sqrt(2) |
    | 1 / sqrt(2), -1 / sqrt(2) |
    )]
    #[dagger(H)]
    H,

    /// Pauli X
    #[mat(
    | 0, 1 |
    | 1, 0 |
    )]
    #[dagger(X)]
    X,

    /// Pauli Y
    #[mat(
    | 0, -i |
    | i,  0 |
    )]
    #[dagger(Y)]
    Y,

    /// Pauli Z
    #[mat(
    | 1,  0 |
    | 0, -1 |
    )]
    #[dagger(Z)]
    Z,

    /// X^t
    #[mat(
    | cos((π/2)*t) * e^((i*(π/2))*t), (-i*sin((π/2)*t)) * e^((i*(π/2))*t) |
    | (-i*sin((π/2)*t)) * e^((i*(π/2))*t), cos((π/2)*t) * e^((i*(π/2))*t) |
    )]
    #[dagger(XPOW { t: -*t })]
    XPOW { t: f64 },

    /// Y^t
    #[mat(
    | cos((π/2)*t) * e^((i*(π/2))*t), -sin((π/2)*t) * e^((i*(π/2))*t) |
    | sin((π/2)*t) * e^((i*(π/2))*t),  cos((π/2)*t) * e^((i*(π/2))*t) |
    )]
    #[dagger(YPOW { t: -*t })]
    YPOW { t: f64 },

    /// Z^t
    #[mat(
    | 1,      0      |
    | 0, e^((i*π)*t) |
    )]
    #[dagger(ZPOW { t: -*t })]
    ZPOW { t: f64 },

    /// S == RZ(pi/2)
    #[mat(
    | 1, 0 |
    | 0, i |
    )]
    #[dagger(SD)]
    S,

    /// S dagger
    #[mat(
    | 1,  0 |
    | 0, -i |
    )]
    #[dagger(S)]
    SD,

    /// T == RZ(pi/4)
    #[mat(
    | 1,    0        |
    | 0, e^((i*π)/4) |
    )]
    #[dagger(TD)]
    T,

    /// T dagger
    #[mat(
    | 1,    0         |
    | 0, e^((-i*π)/4) |
    )]
    #[dagger(T)]
    TD,

    /// Rx [1, 0, 0]
    #[mat(
    | cos(angle/2), -i*sin(angle/2) |
    | -i*sin(angle/2), cos(angle/2) |
    )]
    #[dagger(RX { angle: -*angle })]
    RX { angle: f64 },

    /// Ry [0, 1, 0]
    #[mat(
    | cos(angle/2), -sin(angle/2) |
    | sin(angle/2),  cos(angle/2) |
    )]
    #[dagger(RY { angle: -*angle })]
    RY { angle: f64 },

    /// Rz [0, 0, 1]
    #[mat(
    | e^(-i*(angle/2)), 0 |
    | 0,  e^(i*(angle/2)) |
    )]
    #[dagger(RZ { angle: -*angle })]
    RZ { angle: f64 },

    #[mat(
    | cos(angle/2) - (nz*sin(angle/2))*i, -ny*sin(angle/2) - (nx*sin(angle/2))*i |
    | ny*sin(angle/2) - (nx*sin(angle/2))*i,  cos(angle/2) + (nz*sin(angle/2))*i |
    )]
    #[dagger(RN { nx: *nx, ny: *ny, nz: *nz, angle: -*angle })]
    RN { nx: f64, ny: f64, nz: f64, angle: f64 },

    #[mat(
    |      cos(theta/2),              -(e^(i*lambda)) * sin(theta/2)     |
    | (e^(i*phi)) * sin(theta/2),  (e^(i*(phi + lambda))) * cos(theta/2) |
    )]
    #[dagger(U { theta: -*theta, phi: -*lambda, lambda: -*phi })]
    U { theta: f64, phi: f64, lambda: f64 },

    /// Phase shift gate
    #[mat(
    | 1,    0        |
    | 0, e^(i*angle) |
    )]
    #[dagger(P { angle: -*angle })]
    P { angle: f64 },

    /// Square root of X gate
    #[mat(
    | 0.5 + 0.5i, 0.5 - 0.5i |
    | 0.5 - 0.5i, 0.5 + 0.5i |
    )]
    #[dagger(VD)]
    V,

    #[mat(
    | 0.5 - 0.5i, 0.5 + 0.5i |
    | 0.5 + 0.5i, 0.5 - 0.5i |
    )]
    #[dagger(V)]
    VD,
}

impl_to_mat!(StandardSingleGate: 2);

impl ToMat2 for StandardSingleGate {
    fn to_mat2(&self) -> Mat2 {
        self.get_matrix()
    }
}

impl IntoUnitary<UnitarySingleGate> for StandardSingleGate {
    fn into_unitary(self) -> UnitarySingleGate {
        UnitarySingleGate(Box::new(self.to_mat2()))
    }
}

impl SingleTargetGate for StandardSingleGate {
    type Operation = StandardSingleOperation;

    fn ident(&self) -> String {
        <Self as Into<&'static str>>::into(*self).to_string()
    }

    fn apply_to(self, target: TargetSingle) -> Self::Operation {
        Self::Operation::new(self, target)
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq,
    GatesDef, IntoStaticStr, EnumVariantNames,
)]
pub enum StandardDoubleGate {
    /// Swap Gate
    #[mat(
    | 1, 0, 0, 0 |
    | 0, 0, 1, 0 |
    | 0, 1, 0, 0 |
    | 0, 0, 0, 1 |
    )]
    #[dagger(SWP)]
    SWP,

    /// iSwap Gate
    #[mat(
    | 1, 0, 0, 0 |
    | 0, 0, i, 0 |
    | 0, i, 0, 0 |
    | 0, 0, 0, 1 |
    )]
    #[dagger(ISWPD)]
    ISWP,

    /// iSwap dagger Gate
    #[mat(
    | 1,  0,  0, 0 |
    | 0,  0, -i, 0 |
    | 0, -i,  0, 0 |
    | 0,  0,  0, 1 |
    )]
    #[dagger(ISWP)]
    ISWPD,

    /// Sqrt Swap Gate
    #[mat(
    | 1,    0,       0,    0 |
    | 0, (1+i)/2, (1-i)/2, 0 |
    | 0, (1-i)/2, (1+i)/2, 0 |
    | 0,    0,       0,    1 |
    )]
    #[dagger(SSWPD)]
    SSWP,

    /// Sqrt Swap Gate
    #[mat(
    | 1,    0,       0,    0 |
    | 0, (1-i)/2, (1+i)/2, 0 |
    | 0, (1+i)/2, (1-i)/2, 0 |
    | 0,    0,       0,    1 |
    )]
    #[dagger(SSWP)]
    SSWPD,

    /// Controlled X gate (CNOT)
    #[mat(
    | 1, 0, 0, 0 |
    | 0, 1, 0, 0 |
    | 0, 0, 0, 1 |
    | 0, 0, 1, 0 |
    )]
    #[dagger(CX)]
    CX,

    /// Controlled Z gate
    #[mat(
    | 1, 0, 0,  0 |
    | 0, 1, 0,  0 |
    | 0, 0, 1,  0 |
    | 0, 0, 0, -1 |
    )]
    #[dagger(CZ)]
    CZ,

    /// CPhase Gate
    #[mat(
    | 1, 0, 0,  0 |
    | 0, 1, 0,  0 |
    | 0, 0, 1,  0 |
    | 0, 0, 0, e^(i*angle) |
    )]
    #[dagger(CP { angle: -*angle })]
    CP { angle: f64 },

    /// Sqrt iSwap Gate
    #[mat(
    | 1,      0,       0,      0 |
    | 0, 1/sqrt(2), i/sqrt(2), 0 |
    | 0, i/sqrt(2), 1/sqrt(2), 0 |
    | 0,      0,       0,      1 |
    )]
    #[dagger(SISWPD)]
    SISWP,

    /// Sqrt iSwap Gate
    #[mat(
    | 1,      0,         0,      0 |
    | 0,  1/sqrt(2), -i/sqrt(2), 0 |
    | 0, -i/sqrt(2),  1/sqrt(2), 0 |
    | 0,      0,         0,      1 |
    )]
    #[dagger(SISWP)]
    SISWPD,
}

impl_to_mat!(StandardDoubleGate: 4);

impl ToMat4 for StandardDoubleGate {
    fn to_mat4(&self) -> Mat4 {
        self.get_matrix()
    }
}

impl IntoUnitary<UnitaryDoubleGate> for StandardDoubleGate {
    fn into_unitary(self) -> UnitaryDoubleGate {
        UnitaryDoubleGate(Box::new(self.to_mat4()))
    }
}

impl DoubleTargetGate for StandardDoubleGate {

    type Operation = StandardDoubleOperation;

    fn ident(&self) -> String {
        <Self as Into<&'static str>>::into(*self).to_string()
    }

    fn apply_to(self, target: TargetDouble) -> Self::Operation {
        Self::Operation::new(self, target)
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq,
    GatesDef, IntoStaticStr, EnumVariantNames,
)]
pub enum StandardTripleGate {
    /// Toffoli Gate
    #[mat(
    | 1, 0, 0, 0, 0, 0, 0, 0 |
    | 0, 1, 0, 0, 0, 0, 0, 0 |
    | 0, 0, 1, 0, 0, 0, 0, 0 |
    | 0, 0, 0, 1, 0, 0, 0, 0 |
    | 0, 0, 0, 0, 1, 0, 0, 0 |
    | 0, 0, 0, 0, 0, 1, 0, 0 |
    | 0, 0, 0, 0, 0, 0, 0, 1 |
    | 0, 0, 0, 0, 0, 0, 1, 0 |
    )]
    #[dagger(CCX)]
    CCX,
}

impl_to_mat!(StandardTripleGate: 8);

impl ToMat8 for StandardTripleGate {
    fn to_mat8(&self) -> Mat8 {
        self.get_matrix()
    }
}

impl StandardGate {
    pub fn ident(&self) -> String {
        use StandardGate::*;
        dispatch!(self; Single | Double | Triple => |gate| gate.ident())
    }

    pub fn size(&self) -> usize {
        use StandardGate::*;
        dispatch!(self; Single | Double | Triple => |gate| gate.size())
    }

    pub fn as_single(&self) -> StandardSingleGate {
        unwrap!(*self => StandardGate::Single)
    }
}

impl Dagger for StandardGate {
    fn dagger(self) -> Self {
        use StandardGate::*;
        dispatch!(self; Single | Double | Triple => |gate| gate.dagger().into())
    }
}

impl ToMat for StandardGate {
    fn to_mat(&self) -> GateMat {
        use StandardGate::*;
        dispatch!(self; Single | Double | Triple => |gate| gate.to_mat())
    }
}

impl TripleTargetGate for StandardTripleGate {

    type Operation = StandardTripleOperation;

    fn ident(&self) -> String {
        <Self as Into<&'static str>>::into(*self).to_string()
    }

    fn apply_to(self, target: TargetTriple) -> Self::Operation {
        Self::Operation::new(self, target)
    }
}

into_variant!{
    StandardSingleGate => StandardGate::Single => ElementaryGate::Standard;
    StandardDoubleGate => StandardGate::Double => ElementaryGate::Standard;
    StandardTripleGate => StandardGate::Triple => ElementaryGate::Standard;
}

unwrap_variant! {
    StandardSingleGate => StandardGate::Single;
    StandardDoubleGate => StandardGate::Double;
    StandardTripleGate => StandardGate::Triple;
}
