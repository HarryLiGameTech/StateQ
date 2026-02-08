use crate::operation::controlled::ControlledOperation;
use crate::operation::controlled::mux::MuxOperation;
use crate::operation::Operation;
use crate::program::circuit::QuantumCircuit;
use crate::program::pass::Pass;
use crate::{raise_error, use_enum};

/// Decompose multiplex gate into elementary gates.
pub struct DemultiplexPass;

impl Pass for DemultiplexPass {
    fn apply(&mut self, circuit: &mut QuantumCircuit) {
        circuit.flat_replace_operation(|operation| {
            use_enum!(Operation, ControlledOperation, MuxOperation);
            match operation {
                Controlled(Mux(mux_operation)) => {
                    match mux_operation {
                        SingleTarget(operation) => {
                            Some(operation.decompose())
                        }
                        Rotation(operation) => {
                            Some(operation.decompose())
                        }
                        MultiTarget(_) => raise_error! {
                            "`DemultiplexPass` accepts only single multiplex gate"
                        }
                    }
                }
                _ => None
            }
        })
    }
}
