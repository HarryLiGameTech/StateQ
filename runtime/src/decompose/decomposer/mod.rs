mod graph;
mod builder;

use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::decompose::decomposer::builder::ElementaryDecomposerBuilder;
use crate::decompose::decomposer::graph::DecompositionGraph;
use crate::gate::elementary::ElementaryGate;
use crate::operation::elementary::ElementaryOperation;
use crate::operation::elementary::standard::StandardOperation;
use crate::operation::SingleTargetOperation;

type Decomposition = Box<graph::Delegate<ElementaryOperation>>;
type DecomposeResult = Result<Vec<ElementaryOperation>, DecomposeError>;

pub struct DecomposeError(String);

impl Debug for DecomposeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, formatter)
    }
}

impl Display for DecomposeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Unable to decompose gate operation `{}`", self.0)
    }
}

impl Error for DecomposeError {}

/// Decomposer for elementary gates.
///
/// This decomposer is used to automatically decompose elementary gates that are not available
/// in the target quantum device into a list of elementary gates that are available.
///
/// This decomposer is a wrapper around a graph, which contains the list of elementary gates and the
/// decomposition recipe for each gate.
///
/// The graph is populated with the following methods:
/// - `add_gate`: adds a gate to the graph.
/// - `add_decomposition`: adds a decomposition recipe for a gate.
///
/// Once the graph is populated, you can use the `decompose` method to decompose a gate into other
/// elementary gates.
///
pub struct ElementaryGateDecomposer {
    graph: DecompositionGraph<String, ElementaryOperation>,
}

impl ElementaryGateDecomposer {

    pub(super) fn new() -> Self {
        Self { graph: DecompositionGraph::new() }
    }

    pub fn builder() -> ElementaryDecomposerBuilder {
        ElementaryDecomposerBuilder::new()
    }

    /// Adds a gate to the decomposer.
    pub fn add_gate(&mut self, gate: &str, is_valid: bool) {
        self.graph.add_item(gate.to_string(), is_valid);
    }

    /// Adds a decomposition recipe for a gate.
    pub fn add_decomposition(
        &mut self, from: &str, to: Vec<&str>, cost: i32,
        decomposition: Decomposition,
    ) {
        let to_idents = to.iter().map(|s| s.to_string()).collect();
        self.graph.add_recipe(&from.to_string(), to_idents, cost, decomposition);
    }

    /// Return true if the gate is decomposable.
    pub fn is_gate_decomposable(&self, gate: &ElementaryGate) -> bool {
        self.graph.is_decomposable(&gate.ident())
    }

    /// Return true if the gate is available in the target device backend.
    pub fn is_gate_available(&self, gate_ident: &str) -> bool {
        self.graph.is_available(&gate_ident.to_string())
    }

    /// Decompose a gate into a list of elementary gates.
    /// If the gate is not decomposable, return an `DecomposeError`.
    pub fn decompose(&mut self, gate_op: &ElementaryOperation) -> DecomposeResult {
        let gate_ident = gate_op.get_ident();
        let decompose_result = self.graph.execute_decomposition(&gate_ident, gate_op);
        match decompose_result {
            None => Err(DecomposeError(gate_ident)),
            Some(decomposed) => {
                let decomposed = decomposed.iter().map(|gate_op| match self.decompose(gate_op) {
                    Ok(vec) => vec,
                    Err(_) => vec![gate_op.clone()],
                }).fold(Vec::<ElementaryOperation>::new(), |acc, ele| {
                    acc.into_iter().chain(ele.into_iter()).collect()
                });
                Ok(decomposed)
            }
        }
    }
}
