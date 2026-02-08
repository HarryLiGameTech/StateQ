use crate::program::pass::cond_ctrl_decomposition::ConditionalCtrlDecompositionPass;
use crate::program::pass::demutiplex::DemultiplexPass;
use crate::program::pass::elementary_decomposition::ElementaryDecompositionPass;
use crate::program::pass::multiplexed_optimization::MultiplexOptimizationPass;
use crate::program::pass::remove_identity::RemoveIdentityPass;
use crate::program::QuantumProgramContext;

pub struct QuantumProgramContextBuilder {
    program_ctx: QuantumProgramContext,
}

impl QuantumProgramContextBuilder {

    pub fn new() -> Self {
        Self { program_ctx: QuantumProgramContext::default() }
    }

    pub fn default_passes(&mut self) {
        self.program_ctx.add_pass(MultiplexOptimizationPass);
        self.program_ctx.add_pass(ConditionalCtrlDecompositionPass);
        self.program_ctx.add_pass(DemultiplexPass);
        self.program_ctx.add_pass(RemoveIdentityPass);
        self.program_ctx.add_pass(ElementaryDecompositionPass);
        self.program_ctx.add_pass(RemoveIdentityPass);
    }

    pub fn build(mut self) -> QuantumProgramContext {
        self.program_ctx
    }
}
