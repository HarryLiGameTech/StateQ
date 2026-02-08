use crate::operation::Operation;
use crate::program::circuit::QuantumCircuit;
use crate::program::pass::Pass;
use crate::{QIVM_INSTANCE, raise_error};

/// Decompose elementary gates with the decomposer.
pub struct ElementaryDecompositionPass;

impl Pass for ElementaryDecompositionPass {
    fn apply(&mut self, circuit: &mut QuantumCircuit) {
        let mut qivm = QIVM_INSTANCE.lock().unwrap();
        while circuit.elementary_all(|op| qivm.is_gate_available(&op.get_ident())) {
            circuit.flat_replace_operation(|op| {
                match op {
                    Operation::Elementary(op)
                    if !qivm.is_gate_available(&op.get_ident()) => {
                        Some(qivm.decompose_elementary(&op.clone()))
                    }
                    _ => raise_error!("`ElementaryDecompositionPass` accepts only elementary gates")
                }
            })
        }
    }
}

