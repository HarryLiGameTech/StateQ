use std::f64::consts::PI;
use num::complex::Complex64;
use crate::algebra::Mat2;
use crate::gate::custom::CustomSingleGate;
use crate::gate::standard::StandardSingleGate::{P, X};
use crate::operation::controlled::multi_ctrl::MultiCtrlSingleTargetOperation;
use crate::operation::controlled::single_ctrl::CtrlSingleTargetOperation;
use crate::qubit::qubit_set::QubitSet;
use crate::qubits;

#[test]
fn test_abc_decomposition() {
    let op = CtrlSingleTargetOperation::new(
        CustomSingleGate::new("w".to_string(), Mat2::new(
            1f64.into(), 0f64.into(),
            0f64.into(), Complex64::new(0.38268343236508984, 0.9238795325112867),
        ), vec![]), 0, 1
    );
    let decomposed = op.decompose();
    for op in decomposed.iter() {
        println!("{:?}", op);
    }
}

#[test]
fn test_network_decomposition() {
    let op = MultiCtrlSingleTargetOperation::new(P { angle: PI * 3.0 / 4.0 }.into(), QubitSet::from(vec![0, 1]), 2);
    let decomposed = op.decompose();

    for op in decomposed.iter() {
        println!("{:?}", op);
    }
}
