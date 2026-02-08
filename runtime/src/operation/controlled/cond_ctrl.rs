use crate::gate::elementary::{ElementaryGate, SingleGate};
use crate::gate::{DoubleTargetGate, SingleTargetGate, TripleTargetGate};
use crate::gate::standard::{StandardDoubleGate, StandardGate};
use crate::gate::standard::StandardDoubleGate::{CP, CX, CZ};
use crate::gate::standard::StandardSingleGate::{P, X, Z};
use crate::{into_variant, qubits};
use crate::algebra::{is_identity, ToMat2};
use crate::operation::controlled::ControlledOperation;
use crate::operation::controlled::mux::multi_target::MultiTargetMuxOperation;
use crate::operation::elementary::ElementaryOperation;
use crate::operation::{Operation, TargetMultiple, TargetSingle};
use crate::operation::controlled::multi_ctrl::MultiCtrlSingleTargetOperation;
use crate::qubit::ctrl_qubit_set::ControlQubitSet;
use crate::qubit::qubit_accessor::QubitAccessor;

#[derive(Clone, Debug)]
pub struct ConditionalCtrlOperation {
    gate: ElementaryGate,
    ctrl: ControlQubitSet,
    target: TargetMultiple,
}

impl ConditionalCtrlOperation {
    pub fn new(gate: ElementaryGate, ctrl: ControlQubitSet, target: QubitAccessor) -> Self {
        Self { gate, ctrl, target }
    }

    pub fn dispatch(&self) -> Vec<ConditionalCtrlSingleTargetOperation> {
        if self.gate.size() == 1 {
            let single_gate: SingleGate = self.gate.clone().try_into().unwrap();
            vec![ConditionalCtrlSingleTargetOperation::new(
                single_gate, self.ctrl.clone(), self.target[0]
            )]
        } else {
            match self.gate {
                ElementaryGate::Standard(StandardGate::Double(gate)) => {
                    match gate {
                        StandardDoubleGate::CX => {
                            let mut ctrl = self.ctrl.clone();
                            ctrl.control_one(&qubits![self.target[0]]);
                            vec![ConditionalCtrlSingleTargetOperation::new(
                                X.into(), ctrl, self.target[1]
                            )]
                        }
                        StandardDoubleGate::CZ => {
                            let mut ctrl = self.ctrl.clone();
                            ctrl.control_one(&qubits![self.target[0]]);
                            vec![ConditionalCtrlSingleTargetOperation::new(
                                Z.into(), ctrl, self.target[1]
                            )]
                        }
                        StandardDoubleGate::CP { angle } => {
                            let mut ctrl = self.ctrl.clone();
                            ctrl.control_one(&qubits![self.target[0]]);
                            vec![ConditionalCtrlSingleTargetOperation::new(
                                P { angle }.into(), ctrl, self.target[1]
                            )]
                        }
                        StandardDoubleGate::SWP => {
                            let mut ctrl0 = self.ctrl.clone();
                            ctrl0.control_one(&qubits![self.target[0]]);
                            let mut ctrl1 = self.ctrl.clone();
                            ctrl1.control_one(&qubits![self.target[1]]);
                            vec![
                                ConditionalCtrlSingleTargetOperation::new(
                                    X.into(), ctrl1.clone(), self.target[0]
                                ),
                                ConditionalCtrlSingleTargetOperation::new(
                                    X.into(), ctrl0, self.target[1]
                                ),
                                ConditionalCtrlSingleTargetOperation::new(
                                    X.into(), ctrl1, self.target[0]
                                )
                            ]
                        }
                        _ => {
                            todo!()
                        }
                    }
                }
                _ => {
                    todo!()
                }
            }
        }
    }
}

pub struct ConditionalCtrlSingleTargetOperation {
    gate: SingleGate,
    ctrl: ControlQubitSet,
    target: TargetSingle,
}

impl ConditionalCtrlSingleTargetOperation {
    pub fn new(gate: SingleGate, ctrl: ControlQubitSet, target: TargetSingle) -> Self {
        Self { gate, ctrl, target }
    }

    pub fn simplify_special_gate(&self) -> Option<ElementaryOperation> {
        if self.ctrl.zero_count() > 1 {
            return None;
        }
        match self.ctrl.size() {
            0 => Some(self.gate.clone().apply_to(self.target).into()),
            1 => {
                let ctrl = self.ctrl.ones_vec()[0];
                match self.gate {
                    SingleGate::Standard(X) => Some(CX.apply_to((ctrl, self.target)).into()),
                    SingleGate::Standard(Z) => Some(CZ.apply_to((ctrl, self.target)).into()),
                    SingleGate::Standard(P { angle }) => Some(CP { angle }.apply_to((ctrl, self.target)).into()),
                    _ => None,
                }
            },
            // 2 => {
            //     let ctrls = self.ctrl.ones_vec();
            //     match self.gate {
            //         SingleGate::Standard(X) => Some(CCX.apply_to((ctrls[0], ctrls[1], self.target)).into()),
            //         _ => None,
            //     }
            // }
            _ => None,
        }
    }

    pub fn decompose(&self) -> Vec<ElementaryOperation> {
        if is_identity(&self.gate.to_mat2()) {
            return vec![];
        }
        if let Some(simplified) = self.simplify_special_gate() {
            return vec![simplified];
        }
        let mut operations = Vec::<ElementaryOperation>::new();
        let zero_ctrls = self.ctrl.zeros_vec();
        for zero_ctrl in zero_ctrls.iter() {
            operations.push(X.apply_to(*zero_ctrl).into());
        }
        operations.append(&mut MultiCtrlSingleTargetOperation::new(
            self.gate.clone(), self.ctrl.to_qubit_set(), self.target
        ).decompose());
        for zero_ctrl in zero_ctrls.iter() {
            operations.push(X.apply_to(*zero_ctrl).into());
        }
        operations
    }
}

impl Into<MultiTargetMuxOperation> for ConditionalCtrlSingleTargetOperation {
    fn into(self) -> MultiTargetMuxOperation {
        todo!()
    }
}

into_variant! {
    ConditionalCtrlOperation => ControlledOperation::ConditionalCtrl => Operation::Controlled
}
