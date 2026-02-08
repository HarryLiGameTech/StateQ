use crate::program::circuit::QuantumCircuit;
use crate::program::pass::Pass;
use crate::use_enum;
use crate::gate::standard::StandardDoubleGate;
use crate::gate::standard::StandardDoubleGate::CX;
use crate::operation::elementary::ElementaryOperation;
use crate::operation::elementary::standard::{StandardDoubleOperation, StandardOperation};
use crate::operation::{DoubleTargetOperation, Operation};

/// Decompose SWAP gate into CNOT gates.
pub struct SwapDecompositionPass {}

impl Pass for SwapDecompositionPass {
    fn apply(&mut self, circuit: &mut QuantumCircuit) {
        circuit.flat_replace_operation(|operation| {
            use_enum!(Operation, ElementaryOperation, StandardOperation);
            match operation {
                Elementary(Standard(Double(operation)))
                if matches!(operation.get_gate(), StandardDoubleGate::SWP) => {
                    let (target0, target1) = operation.get_target();
                    Some(vec![
                        StandardDoubleOperation::new(CX, (target0, target1)),
                        StandardDoubleOperation::new(CX, (target1, target0)),
                        StandardDoubleOperation::new(CX, (target0, target1)),
                    ])
                }
                _ => None
            }
        })
    }
}
