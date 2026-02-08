use crate::program::circuit::QuantumCircuit;
use crate::program::pass::Pass;

pub struct MultiplexOptimizationPass;

impl Pass for MultiplexOptimizationPass {
    fn apply(&mut self, circuit: &mut QuantumCircuit) {
        // TODO
    }
}
