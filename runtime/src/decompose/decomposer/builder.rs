use strum::VariantNames;
use crate::backend::is_gate_available;
use crate::decompose::decomposer::ElementaryGateDecomposer;
use crate::decompose::single::{decompose_single, zyz_decompose};
use crate::gate::standard::StandardSingleGate;
use crate::gate::unitary::{DOUBLE_UNITARY_IDENT, SINGLE_UNITARY_IDENT};
use crate::algebra::ToMat2;
use crate::operation::elementary::{ElementaryOperation, SingleOperation};
use crate::operation::elementary::unitary::UnitarySingleOperation;
use crate::operation::{ElementaryGateOperation, SingleTargetOperation};
use crate::QIVM_INSTANCE;

pub struct ElementaryDecomposerBuilder {
    decomposer: ElementaryGateDecomposer,
}

impl ElementaryDecomposerBuilder {

    pub fn new() -> Self {
        Self { decomposer: ElementaryGateDecomposer::new() }
    }

    pub fn build(mut self) -> ElementaryGateDecomposer {
        self.add_standard_gates();
        self.add_unitary_gates();
        self.init_std_single_gates_to_unitary();
        self.init_unitary_zyz_decompose();
        self.decomposer
    }

    fn add_unitary_gates(&mut self) {
        self.decomposer.add_gate(SINGLE_UNITARY_IDENT, false);
        self.decomposer.add_gate(DOUBLE_UNITARY_IDENT, false);
    }

    fn add_standard_gates(&mut self) {
        for &gate_ident in StandardSingleGate::VARIANTS {
            if is_gate_available(gate_ident) {
                self.decomposer.add_gate(gate_ident, true);
            } else {
                self.decomposer.add_gate(gate_ident, false);
            }
        }
    }

    /// Transform standard single gates to unitary single gates.
    fn init_std_single_gates_to_unitary(&mut self) {
        for &gate_ident in StandardSingleGate::VARIANTS {
            if gate_ident == "I" {
                self.decomposer.add_decomposition(gate_ident, vec![], -1, Box::new(|_| vec![]));
            } else {
                self.decomposer.add_decomposition(
                    gate_ident, vec![SINGLE_UNITARY_IDENT], 0, Box::new(|op| {
                        let single_op: SingleOperation = op.clone().try_into().unwrap();
                        vec![UnitarySingleOperation::from_mat(
                            single_op.to_mat2(), single_op.get_target()
                        ).into()]
                    })
                );
            }
        }
    }

    /// Decompose unitary single gates with ZYZ decomposition.
    fn init_unitary_zyz_decompose(&mut self) {
        self.decomposer.add_decomposition(
            SINGLE_UNITARY_IDENT, vec!["RY", "RZ", "P"], 4, Box::new(|op| {
                let single_op: SingleOperation = op.clone().try_into().unwrap();
                decompose_single(&single_op)
            })
        )
    }
}
