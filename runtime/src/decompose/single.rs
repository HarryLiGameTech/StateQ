use approx::AbsDiffEq;
use libm::{cos, sin};
use nalgebra::ComplexField;
use num::complex::Complex64;
use num::traits::FloatConst;
use crate::ElementaryOperation;
use crate::gate::rotation::Rotation;
use crate::gate::rotation::Rotation::{Ry, Rz};
use crate::gate::standard::StandardSingleGate;
use crate::algebra::{EPSILON, GateMat, Mat2, phase_angle, su};
use crate::operation::SingleTargetOperation;
use crate::qubit::qubit_accessor::QubitAccessor;
use crate::qubits;
use crate::gate::elementary::ElementaryGate;

pub struct EulerDecomposition(pub Rotation, pub Rotation, pub Rotation, pub f64);

pub fn decompose_single(gate_op: &impl SingleTargetOperation) -> Vec<ElementaryOperation> {
    let EulerDecomposition(r0, r1, r2, _) = zyz_decompose(&gate_op.to_mat2());
    let gates = vec![
        r0.into(), r1.into(), r2.into(),
        // StandardSingleGate::P { angle: ph }.into()
    ];
    gates.into_iter().map(|gate: ElementaryGate| {
        gate.apply_to(qubits![gate_op.get_target()])
    }).collect()
}

/// Decompose a 1-qubit gate to sequence of Z, Y, and Z rotations, and a phase
pub fn zyz_decompose(gate: &Mat2) -> EulerDecomposition {
    let mut alpha = phase_angle(gate);
    let su = su::<2>(gate);
    // φ = {
    //  2arccos(|U_00|) if |U_00| >= |U_01|
    //  2arcsin(|U_01|) if |U_00| <  |U_01|
    // }
    let phi = if su[(0, 0)].abs() > su[(1, 0)].abs() {
        -2.0 * su[(0, 0)].abs().min(1.0).acos()
    } else {
        -2.0 * su[(1, 0)].abs().min(1.0).asin()
    };

    let (theta, lambda) = {
        let theta_plus_lambda = if (phi / 2.0).cos().abs_diff_eq(&0.0, EPSILON) {
            0.0 // if cos(φ/2) == 0 then θ + λ = 0
        } else {
            // else θ + λ = 2arctan2(Im(U_11 / cos(φ/2), Re(U_11 / cos(φ/2))
            let z = su[(1, 1)] / cos(0.5 * phi);
            2.0 * f64::atan2(z.im, z.re)
        };
        let theta_minus_lambda = if (phi / 2.0).sin().abs_diff_eq(&0.0, EPSILON) {
            0.0 // if sin(φ*1/2) == 0 then θ - λ = 0
        } else {
            // else θ + λ = 2arctan2(Im(U_10 / sin(φ/2), Re(U_10 / sin(φ/2))
            let z = su[(1, 0)] / sin(0.5 * phi);
            2.0 * f64::atan2(z.im, z.re)
        };
        // θ = ((θ + λ) + (θ - λ)) / 2
        let theta = (theta_plus_lambda + theta_minus_lambda) / 2f64;
        // λ = ((θ + λ) - (θ - λ)) / 2
        let lambda = (theta_plus_lambda - theta_minus_lambda) / 2f64;

        (theta, lambda)
    };

    // alpha += (theta + lambda) / 2.0;

    assert!(!lambda.is_nan() && !phi.is_nan() && !theta.is_nan() && !alpha.is_nan());
    EulerDecomposition(Rz(lambda), Ry(phi), Rz(theta), alpha)
}
