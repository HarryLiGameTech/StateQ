use crate::operation::controlled::ControlledOperation;
use crate::operation::Operation;
use crate::program::circuit::QuantumCircuit;
use crate::program::pass::Pass;
use crate::use_enum;

pub struct ConditionalCtrlDecompositionPass;

impl Pass for ConditionalCtrlDecompositionPass {
    fn apply(&mut self, circuit: &mut QuantumCircuit) {
        circuit.flat_replace_operation(|op| {
            use_enum!(Operation, ControlledOperation);
            match op {
                Controlled(ConditionalCtrl(op)) => {
                    Some(op.dispatch().into_iter().flat_map(|op| op.decompose()).collect())
                }
                _ => None
            }
        })
    }
}
