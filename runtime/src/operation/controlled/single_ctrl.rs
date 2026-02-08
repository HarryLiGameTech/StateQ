use crate::gate::elementary::{ElementaryGate, SingleGate};
use crate::gate::{Dagger, DoubleTargetGate, SingleTargetGate};
use crate::gate::standard::StandardSingleGate::*;
use crate::gate::standard::StandardDoubleGate::*;
use crate::qubit::QubitAddr;
use crate::qubit::qubit_accessor::QubitAccessor;
use crate::decompose::single::EulerDecomposition;
use crate::decompose::single::zyz_decompose;
use crate::gate::rotation::Rotation;
use crate::gate::standard::{StandardGate, StandardSingleGate};
use crate::algebra::ToMat2;
use crate::operation::elementary::ElementaryOperation;
use crate::macros::Unwrap;

#[derive(Clone)]
pub struct CtrlMultiTargetOperation {
    pub gate: ElementaryGate,
    pub ctrl: QubitAddr,
    pub target: QubitAccessor,
}

impl CtrlMultiTargetOperation {
    pub fn size(&self) -> usize {
        self.target.size()
    }
}

impl Dagger for CtrlMultiTargetOperation {
    fn dagger(self) -> Self {
        Self {
            gate: self.gate.dagger(),
            ctrl: self.ctrl,
            target: self.target,
        }
    }
}

/// Single controlled single target gate
#[derive(Clone)]
pub struct CtrlSingleTargetOperation {
    gate: SingleGate,
    ctrl: QubitAddr,
    target: QubitAddr,
}

impl CtrlSingleTargetOperation {
    pub fn new(gate: impl Into<SingleGate>, ctrl: QubitAddr, target: QubitAddr) -> Self {
        Self { gate: gate.into(), ctrl, target }
    }

    pub fn decompose(&self) -> Vec<ElementaryOperation> {
        match self.gate {
            SingleGate::Standard(X) => vec![CX.apply_to((self.ctrl, self.target)).into()],
            SingleGate::Standard(Z) => vec![CZ.apply_to((self.ctrl, self.target)).into()],
            SingleGate::Standard(P { angle }) => vec![
                CP { angle }.apply_to((self.ctrl, self.target)).into()
            ],
            _ => self.abc_decompose(),
        }
    }
}

impl Dagger for CtrlSingleTargetOperation {
    fn dagger(self) -> Self {
        Self { gate: self.gate.dagger(), ..self }
    }
}

impl CtrlSingleTargetOperation {
    /// Decompose controlled-U gate
    /// U = exp(iα) A X B X C
    fn abc_decompose(&self) -> Vec<ElementaryOperation> {
        use Rotation::*;
        let mat = self.gate.to_mat2();
        if let EulerDecomposition(Rz(lambda), Ry(phi), Rz(theta), alpha) = zyz_decompose(&mat) {
            let apply_to_target = |gates: Vec<StandardGate>| -> Vec<ElementaryOperation> {
                gates.into_iter().rev().map(|gate| {
                    let gate: StandardSingleGate = gate.unwrap();
                    gate.apply_to(self.target).into()
                }).collect()
            };
            let a = apply_to_target(vec![
                RZ { angle: lambda }.into(),
                RY { angle: 0.5 * phi }.into(),
            ]);
            let b = apply_to_target(vec![
                RY { angle: -0.5 * phi }.into(),
                RZ { angle: -0.5 * (theta + lambda) }.into(),
            ]);
            let c = apply_to_target(vec![
                RZ { angle: 0.5 * (theta - lambda) }.into()
            ]);
            let cnot: ElementaryOperation = CX.apply_to((self.ctrl, self.target)).into();
            c.into_iter()
                .chain(vec![cnot.clone()])
                .chain(b.into_iter())
                .chain(vec![cnot])
                .chain(a.into_iter())
                .chain(vec![P { angle: alpha }.apply_to(self.ctrl).into()])
                .collect()
        } else { unreachable!() }
    }
}
