use std::mem::swap;
use std::ops::{Add, AddAssign};
use crate::bytecode::instruction::{InstrParam, Instruction, PrimitiveOpCode};
use crate::operation::Operation;
use crate::operation::elementary::ElementaryOperation;
use crate::qubit::QubitAddr;
use crate::raise_error;

/// A quantum circuit is a sequence of operations
/// which can be compiled into a sequence of instructions
/// to be executed on a quantum computer.
/// The `CircuitOperation` type is a tuple of an operation and a stack top pointer.
#[derive(Default, Debug)]
pub struct QuantumCircuit {
    /// The sequence of operations.
    pub operations: Vec<CircuitOperation>,
}

impl QuantumCircuit {

    pub fn from_operations(operations: Vec<CircuitOperation>) -> Self {
        Self { operations }
    }

    /// Push an operation into the circuit.
    pub fn push_op(&mut self, op: impl Into<Operation>, stack_top: QubitAddr) {
        self.operations.push(CircuitOperation::new(op.into(), stack_top));
    }

    /// Compile the circuit into a sequence of instructions.
    pub fn compile(&self) -> Vec<Instruction> {
        let mut instructions = Vec::<Instruction>::new();
        let mut last_stack_top: QubitAddr = 0;
        let mut qubits_alloc: QubitAddr = 0;
        for CircuitOperation { operation, stack_top } in &self.operations {
            if *stack_top < last_stack_top {
                instructions.push(Instruction::Primitive {
                    opcode: PrimitiveOpCode::Reset,
                    params: (*stack_top .. last_stack_top).map(|qubit| {
                        InstrParam::UInt(qubit as u64)
                    }).collect(),
                });
            } else if *stack_top > qubits_alloc {
                qubits_alloc = *stack_top;
            }
            instructions.push(operation.clone().into());
            last_stack_top = *stack_top;
        }
        // Alloc qubits
        instructions.insert(0, Instruction::Primitive {
            opcode: PrimitiveOpCode::Alloc,
            params: vec![InstrParam::UInt(qubits_alloc as u64)],
        });
        instructions
    }

    pub fn reverse(&mut self) {
        self.operations.reverse();
    }

    pub fn reversed(self) -> Self {
        Self { operations: self.operations.into_iter().rev().collect() }
    }

    pub fn flat_replace<F>(&mut self, mut transform: F)
    where
        F: FnMut(&CircuitOperation) -> Option<Vec<CircuitOperation>>
    {
        let mut operations = Vec::<CircuitOperation>::new();
        // Swap `self.operations` with an empty `Vec<T>`
        //  to avoid cloning the whole `self.operations`
        swap(&mut operations, &mut self.operations);
        self.operations = operations.into_iter().flat_map(|op| {
            transform(&op).unwrap_or(vec![op])
        }).collect::<Vec<CircuitOperation>>();
    }

    pub fn flat_replace_operation<F, OP>(&mut self, mut transform: F)
    where
        F: FnMut(&Operation) -> Option<Vec<OP>>,
        OP: Into<Operation>,
    {
        self.flat_replace(|operation| {
            let CircuitOperation { operation, stack_top } = operation;
            transform(operation).map(|decomposed| {
                decomposed.into_iter().map(|op| {
                    CircuitOperation::new(op.into(), *stack_top)
                }).collect::<Vec<CircuitOperation>>()
            })
        });
    }

    /// Return true if all operations satisfy the predicate.
    pub fn all(&mut self, mut predict: impl FnMut(&Operation, QubitAddr) -> bool) -> bool {
        self.operations.iter().all(|op| predict(&op.operation, op.stack_top))
    }

    /// Return true if all elementary operations satisfy the predicate.
    /// Raise an error if there is a non-elementary operation.
    pub fn elementary_all(
        &mut self, mut predict: impl FnMut(&ElementaryOperation) -> bool
    ) -> bool {
        self.all(|operation, _| {
            if let Operation::Elementary(op) = &operation {
                predict(op)
            } else {
                raise_error!("Non-elementary operation");
            }
        })
    }

    /// Return true if any operation satisfies the predicate.
    pub fn any(&mut self, mut predict: impl FnMut(&Operation, QubitAddr) -> bool) -> bool {
        self.operations.iter().any(|op| predict(&op.operation, op.stack_top))
    }

    /// Return true if any elementary operation satisfies the predicate.
    /// Raise an error if there is a non-elementary operation.
    pub fn elementary_any(
        &mut self, mut predict: impl FnMut(&ElementaryOperation) -> bool
    ) -> bool {
        self.any(|operation, _| {
            if let Operation::Elementary(op) = &operation {
                predict(op)
            } else {
                raise_error!("Non-elementary operation");
            }
        })
    }
}

impl Add for QuantumCircuit {
    type Output = Self;
    fn add(mut self, mut other: Self) -> Self {
        self.operations.extend(other.operations);
        self
    }
}

impl AddAssign for QuantumCircuit {
    fn add_assign(&mut self, mut other: Self) {
        self.operations.extend(other.operations);
    }
}

/// A circuit operation is a tuple of an operation and a stack top pointer.
#[derive(Clone, Debug)]
pub struct CircuitOperation {
    /// The operation to be executed.
    pub operation: Operation,
    /// The index of the top qubit in the stack.
    pub stack_top: QubitAddr,
}

impl CircuitOperation {
    pub fn new(operation: Operation, stack_top: QubitAddr) -> Self {
        Self { operation, stack_top }
    }
}

/// Convert a circuit operation into an instruction.
impl Into<Instruction> for CircuitOperation {
    fn into(self) -> Instruction {
        self.operation.into()
    }
}
