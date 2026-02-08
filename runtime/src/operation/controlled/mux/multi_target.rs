use crate::gate::Dagger;
use crate::gate::elementary::ElementaryGate;
use crate::operation::elementary::ElementaryOperation;
use crate::operation::Operation;
use crate::qubit::qubit_accessor::QubitAccessor;
use crate::{into_variant, raise_error};
use crate::operation::controlled::ControlledOperation;
use crate::operation::controlled::mux::MuxOperation;

#[derive(Clone, Debug)]
pub struct MultiTargetMuxOperation {
    pub gates: Vec<ElementaryGate>,
    pub ctrl: QubitAccessor,
    pub target: QubitAccessor,
}

impl MultiTargetMuxOperation {
    pub fn new(gates: Vec<ElementaryGate>, ctrl: QubitAccessor, target: QubitAccessor) -> Self {
        if gates.len() != 2usize.pow(ctrl.size() as u32) {
            raise_error!(
                "Invalid control size, expected: {}, actual: {}",
                2usize.pow(ctrl.size() as u32), gates.len()
            );
        }
        for gate in gates.iter() {
            if gate.size() != 1 {
                raise_error!("Invalid gate size, expected: 1, actual: {}", gate.size());
            }
        }
        Self { gates, ctrl, target }
    }

    pub fn gates_vec(&self) -> Vec<ElementaryGate> {
        self.gates.clone()
    }

    pub fn decompose(&self) -> Vec<Operation> {
        todo!()
    }

    fn demultiplex(&self) -> Vec<ElementaryOperation> {
        todo!()
    }
}

impl Dagger for MultiTargetMuxOperation {
    fn dagger(self) -> Self {
        Self {
            gates: self.gates.into_iter().map(ElementaryGate::dagger).collect(),
            ctrl: self.ctrl.clone(),
            target: self.target,
        }
    }
}

into_variant! {
    MultiTargetMuxOperation 
        => MuxOperation::MultiTarget 
        => ControlledOperation::Mux 
        => Operation::Controlled;
}
