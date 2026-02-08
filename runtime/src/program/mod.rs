pub mod builder;
mod circuit;
mod pass;

#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::collections::{VecDeque};
use std::rc::Rc;
use crate::gate::elementary::{ElementaryGate, SingleGate};
use crate::gate::{Dagger, SingleTargetGate};
use crate::algebra::GateMat;
use crate::qubit::QubitAddr;
use crate::qubit::qubit_accessor::QubitAccessor;
use crate::{QIVM_INSTANCE, qubits, raise_error};
use crate::backend::RawExecuteResult;
use crate::bytecode::ByteCode;
use crate::bytecode::instruction::{InstrParam, Instruction, PrimitiveOpCode};
use crate::gate::custom::CustomGate;
use crate::gate::standard::StandardSingleGate::X;
use crate::measurement::{MeasurementResult, MeasurementResultEntry};
use crate::operation::controlled::cond_ctrl::ConditionalCtrlOperation;
use crate::operation::Operation;
use crate::program::circuit::QuantumCircuit;
use crate::program::pass::Pass;
use crate::qubit::ctrl_qubit_set::ControlQubitSet;
use crate::qubit::qubit_set::QubitSet;

pub type QubitAccessorRef = Rc<RefCell<QubitAccessor>>;

pub struct QuantumProgramContext {
    ctrl_qubits: ControlQubitSet,
    ctrl_qubits_stack: VecDeque<ControlQubitSet>,
    qubits_stack: VecDeque<QuantumStackFrame>,
    dagger_stack: VecDeque<QuantumCircuit>,
    stack_top: QubitAddr,
    is_dagger: bool,
    circuit: QuantumCircuit,
    measurement: QubitAccessor,
    transpile_passes: Vec<Box<dyn Pass>>,
    result: Option<MeasurementResult>,
}

impl Default for QuantumProgramContext {
    fn default() -> Self {
        Self {
            ctrl_qubits: ControlQubitSet::new(),
            ctrl_qubits_stack: VecDeque::new(),
            qubits_stack: VecDeque::new(),
            dagger_stack: VecDeque::new(),
            stack_top: 0,
            is_dagger: false,
            circuit: QuantumCircuit::default(),
            measurement: QubitAccessor::new(),
            transpile_passes: vec![],
            result: None,
        }
    }
}

impl QuantumProgramContext {

    pub fn enter(&mut self) {
        self.qubits_stack.push_back(QuantumStackFrame::new(self.stack_top));
    }

    pub fn exit(&mut self) {
        self.stack_top = self.qubits_stack.pop_back().unwrap_or_else(|| {
            raise_error!("Invalid exit operation: quantum stack underflow");
        }).stack_base;
    }

    pub fn add_pass(&mut self, pass: impl Pass + 'static) {
        self.transpile_passes.push(Box::new(pass));
    }

    pub fn alloc(&mut self, size: usize) -> QubitAccessorRef {
        let new_stack_top = self.stack_top + size as QubitAddr;
        let accessor = QubitAccessor::range(self.stack_top, new_stack_top - 1);
        self.stack_top = new_stack_top;
        self.qubits_stack.back_mut().unwrap_or_else(|| {
            raise_error!("Invalid allocation: quantum stack is empty")
        }).alloc(accessor)
    }

    pub fn add_qubit_accessor(&mut self, accessor: QubitAccessorRef) {
        self.qubits_stack.back_mut().unwrap_or_else(|| {
            raise_error!("Invalid allocation: quantum stack is empty")
        }).add_qubit_accessor(accessor);
    }

    pub fn encode(&mut self, accessor: &QubitAccessor, value: u32) {
        accessor.to_vec().iter().enumerate().for_each(|(i, qubit)| {
            if value >> i == 1 {
                self.push(X, qubits![*qubit]);
            }
        });
    }

    fn push_op(&mut self, op: impl Into<Operation>) {
        if self.dagger_stack.is_empty() {
            self.circuit.push_op(op, self.stack_top);
        } else {
            self.dagger_stack.back_mut().unwrap().push_op(op, self.stack_top);
        }
    }

    pub fn push(&mut self, gate: impl Into<ElementaryGate>, target: QubitAccessor) {
        if target.iter().any(|&qubit| self.ctrl_qubits.contains(qubit)) {
            raise_error!("Invalid operation: target qubit is controlled");
        }
        let gate: ElementaryGate = if self.is_dagger {
            gate.into().dagger()
        } else {
            gate.into()
        };
        if self.ctrl_qubits.is_empty() {
            self.push_op(gate.apply_to(target));
        } else {
            self.push_op(ConditionalCtrlOperation::new(gate, self.ctrl_qubits.clone(), target));
        };
    }

    pub fn control(&mut self, ctrl: QubitAccessor, condition: bool) {
        self.ctrl_qubits.control(&ctrl, condition);
    }

    pub fn decontrol(&mut self, ctrl: QubitAccessor) {
        self.ctrl_qubits.decontrol(&ctrl);
    }

    pub fn pause_ctrl(&mut self) {
        let mut ctrl = ControlQubitSet::new();
        std::mem::swap(&mut self.ctrl_qubits, &mut ctrl);
        self.ctrl_qubits_stack.push_back(ctrl);
    }

    pub fn restore_ctrl(&mut self) {
        if self.ctrl_qubits.size() != 0 {
            raise_error!("Invalid restore_ctrl operation: current control qubits are not empty");
        } else if self.ctrl_qubits_stack.is_empty() {
            raise_error!("Invalid restore_ctrl operation: control qubits stack underflow");
        }
        self.ctrl_qubits = self.ctrl_qubits_stack.pop_back().unwrap();
    }

    pub fn is_dagger(&self) -> bool {
        self.is_dagger
    }

    pub fn begin_dagger(&mut self) {
        self.dagger_stack.push_back(QuantumCircuit::default());
        self.is_dagger = !self.is_dagger;
    }

    pub fn end_dagger(&mut self) {
        let dagger_section = self.dagger_stack.pop_back().unwrap_or_else(|| {
            raise_error!("Invalid end_dagger operation: dagger stack underflow");
        });
        let base_circuit: &mut QuantumCircuit = if self.dagger_stack.is_empty() {
            &mut self.circuit
        } else {
            self.dagger_stack.back_mut().unwrap()
        };
        *base_circuit += dagger_section.reversed();
        self.is_dagger = !self.is_dagger;
    }

    pub fn push_custom(
        &mut self, ident: String, mat: GateMat,
        params: Vec<u64>, target: QubitAccessor
    ) {
        let target_size = mat.target_size();
        if target.size() != target_size {
            raise_error!("Invalid target size, expected: {}, actual: {}", target_size, target.size());
        } else {
            todo!()
        }
    }

    pub fn push_custom_builtin(
        &mut self, ident: String, params: Vec<u64>,
        size: usize, target: QubitAccessor
    ) {
        if target.size() != size {
            raise_error!("Invalid target size, expected: {}, actual: {}", size, target.size());
        } else if !QIVM_INSTANCE.lock().unwrap().is_gate_available(&ident) {
            raise_error!("Invalid primitive gate `{}` on target platform", ident);
        } else {
            todo!()
        }
    }

    pub fn measure(&mut self, targets: QubitAccessor) {
        self.measurement = targets;
    }

    pub fn transpile(&mut self) {
        self.transpile_passes.iter_mut().for_each(|pass| {
            pass.apply(&mut self.circuit)
        });
    }

    pub fn compile_circuit(&mut self) -> Vec<Instruction> {
        self.transpile();
        let mut instructions = self.circuit.compile();
        instructions.push(Instruction::Primitive {
            opcode: PrimitiveOpCode::Measure,
            params: self.measurement.iter().map(|qubit| {
                InstrParam::UInt(*qubit as u64)
            }).collect(),
        });
        instructions
    }

    pub fn compile_bytecode(&mut self) -> ByteCode {
        self.compile_circuit().into()
    }

    pub fn set_measurement_result(&mut self, result: MeasurementResult) {
        self.result = Some(result);
    }

    pub fn get_measurement_result(&self) -> Option<MeasurementResult> {
        self.result.clone()
    }
}

struct QuantumStackFrame {
    stack_base: QubitAddr,
    qubit_accessors: Vec<QubitAccessorRef>,
}

impl QuantumStackFrame {
    pub fn new(stack_base: QubitAddr) -> Self {
        Self {
            stack_base,
            qubit_accessors: Vec::new()
        }
    }

    pub fn alloc(&mut self, accessor: QubitAccessor) -> QubitAccessorRef {
        let accessor_ref: QubitAccessorRef = Rc::new(accessor.into());
        self.qubit_accessors.push(accessor_ref.clone());
        accessor_ref
    }

    pub fn add_qubit_accessor(&mut self, accessor: QubitAccessorRef) {
        self.qubit_accessors.push(accessor);
    }
}
