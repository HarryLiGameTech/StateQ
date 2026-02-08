use crate::operation::elementary::ElementaryOperation;
use crate::operation::elementary::standard::StandardOperation;
use crate::operation::{Operation, SingleTargetOperation};
use crate::program::circuit::QuantumCircuit;
use crate::program::pass::Pass;
use crate::gate::elementary::is_identity;
use crate::use_enum;

pub struct RemoveIdentityPass;

impl Pass for RemoveIdentityPass {
    fn apply(&mut self, circuit: &mut QuantumCircuit) {
        circuit.flat_replace_operation(|operation| {
            use_enum!(Operation, ElementaryOperation, StandardOperation);
            if let Elementary(operation) = operation {
                if is_identity(&operation.get_gate()) {
                    return Some(Vec::<ElementaryOperation>::new())
                }
            }
            None
        })
    }
}
