#![allow(mixed_script_confusables)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::from_over_into)]
#![allow(clippy::missing_safety_doc)]

extern crate core;
extern crate gates_def;

use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::backend::get_available_qubits;
use crate::decompose::decomposer::ElementaryGateDecomposer;
use crate::gate::elementary::ElementaryGate;
use crate::operation::elementary::ElementaryOperation;

pub mod backend;
pub mod runtime_api;

mod qubit;
mod gate;
mod decompose;
mod algebra;
mod macros;
mod bytecode;
mod measurement;
mod operation;
mod program;
// mod experimental;

struct QuantumInterfaceVirtualMachine {
    decomposer: ElementaryGateDecomposer,
    available_qubits: usize,
}

lazy_static! {
    static ref QIVM_INSTANCE: Mutex<QuantumInterfaceVirtualMachine> = Mutex::new(
        QuantumInterfaceVirtualMachine::init()
    );
}

impl QuantumInterfaceVirtualMachine {
    pub fn init() -> Self {
        let available_qubits = get_available_qubits();
        let mut decomposer = ElementaryGateDecomposer::builder().build();
        Self { available_qubits, decomposer }
    }

    pub fn is_gate_available(&self, ident: &str) -> bool {
        self.decomposer.is_gate_available(ident)
    }

    pub fn decompose_elementary(&mut self, gate_op: &ElementaryOperation) -> Vec<ElementaryOperation> {
        match self.decomposer.decompose(gate_op) {
            Ok(gate_ops) => gate_ops,
            Err(err) => raise_error!("{}", err),
        }
    }

    pub fn is_gate_decomposable(&self, gate: &ElementaryGate) -> bool {
        self.decomposer.is_gate_decomposable(gate)
    }
}
