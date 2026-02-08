use enum_dispatch::enum_dispatch;
use strum_macros::{EnumVariantNames, IntoStaticStr};
use proc_macro::GateDef;
use crate::algebra::{Mat, DMat, ToDMat};
use crate::{impl_single_gate, into_variant};
use crate::operation::{Dagger, Operation, SingleOperation, StaticOperation, Targets, ToMat};
use crate::qubit::qubit_vec::QubitVec;
use crate::qubit::QubitAddr;
use crate::qubits;

#[enum_dispatch]
#[derive(
    Debug, Copy, Clone, PartialEq,
    GateDef, IntoStaticStr, EnumVariantNames,
)]
pub enum StandardGate {

    /// Hadamard
    #[mat(
    | 1 / sqrt(2),  1 / sqrt(2) |
    | 1 / sqrt(2), -1 / sqrt(2) |
    )]
    #[dagger(H)]
    H(GateH),

    /// Pauli X
    #[mat(
    | 0, 1 |
    | 1, 0 |
    )]
    #[dagger(X)]
    X(GateX),

    /// Pauli Y
    #[mat(
    | 0, -i |
    | i,  0 |
    )]
    #[dagger(Y)]
    Y(GateY),

    /// Pauli Z
    #[mat(
    | 1,  0 |
    | 0, -1 |
    )]
    #[dagger(Z)]
    Z(GateZ),

    /// X^t
    #[mat(
    | cos((π/2)*t) * e^((i*(π/2))*t), (-i*sin((π/2)*t)) * e^((i*(π/2))*t) |
    | (-i*sin((π/2)*t)) * e^((i*(π/2))*t), cos((π/2)*t) * e^((i*(π/2))*t) |
    )]
    #[dagger(XPOW { t: -self.t })]
    #[params(t)]
    XPOW(GateXPow),

    /// Y^t
    #[mat(
    | cos((π/2)*t) * e^((i*(π/2))*t), -sin((π/2)*t) * e^((i*(π/2))*t) |
    | sin((π/2)*t) * e^((i*(π/2))*t),  cos((π/2)*t) * e^((i*(π/2))*t) |
    )]
    #[dagger(YPOW { t: -self.t })]
    #[params(t)]
    YPOW(GateYPow),

    /// Z^t
    #[mat(
    | 1,      0      |
    | 0, e^((i*π)*t) |
    )]
    #[dagger(ZPOW { t: -self.t })]
    #[params(t)]
    ZPOW(GateZPow),

    // U3
    #[mat(
    | cos(θ/2), -e^(i*λ) * sin(θ/2) |
    | e^(i*φ) * sin(θ/2), e^(i*λ + i*φ) * cos(θ/2) |
    )]
    #[dagger(U3 { theta: self.theta, phi: -self.phi, lambda: -self.lambda })]
    #[params(theta, phi, lambda)]
    U3(GateU3),

    // U3 with Phase
    #[mat(
    | e^(i*α)*cos(θ/2), -e^(i*(α+λ)) * sin(θ/2) |
    | e^(i*(α+φ)) * sin(θ/2), e^(i*((α+λ)+φ)) * cos(θ/2) |
    )]
    #[dagger(U4 { alpha: -self.alpha, theta: self.theta, phi: -self.phi, lambda: -self.lambda })]
    #[params(alpha, theta, phi, lambda)]
    U4(GateU4),
}
