pub mod demutiplex;
pub mod elementary_decomposition;
pub mod swap_decomposition;
pub mod multiplexed_optimization;
pub mod pauli_x_cancellation;
pub mod cond_ctrl_decomposition;
pub mod remove_identity;

use crate::program::circuit::QuantumCircuit;

/// A pass is a transformation that can be applied to a quantum circuit.
/// It is usually used to optimize the circuit or to decompose gates.
/// A pass can be applied to a circuit by calling the `apply` method.
pub trait Pass {
    fn apply(&mut self, circuit: &mut QuantumCircuit);
}
