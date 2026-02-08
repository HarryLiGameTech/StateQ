use bit_set::BitSet;
use crate::qubit::qubit_vec::QubitVec;
use crate::qubit::qubit_set::QubitSet;
use crate::qubit::QubitAddr;

pub enum ControlState {
    None, Zero, One, Both,
}

#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct ControlQubitSet {
    ones: QubitSet,
    zeros: QubitSet,
}

impl ControlQubitSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.ones.is_empty() && self.zeros.is_empty()
    }

    pub fn size(&self) -> usize {
        (self.zeros.0.union(&self.ones.0).collect::<BitSet>()).len()
    }
    
    pub fn one_count(&self) -> usize {
        self.ones.size()
    }
    
    pub fn zero_count(&self) -> usize {
        self.zeros.size()
    }

    pub fn ones_vec(&self) -> Vec<QubitAddr> {
        self.ones.to_vec()
    }

    pub fn zeros_vec(&self) -> Vec<QubitAddr> {
        self.zeros.to_vec()
    }

    pub fn to_vec(&self) -> Vec<(QubitAddr, bool)> {
        let mut vec = self.ones.to_vec().into_iter().map(|qubit| (qubit, true))
            .chain(self.zeros.to_vec().into_iter().map(|qubit| (qubit, false)))
            .collect::<Vec<(QubitAddr, bool)>>();
        vec.sort_by_key(|(qubit, bool)| *qubit);
        vec
    }

    pub fn to_qubit_set(&self) -> QubitSet {
        QubitSet(self.ones.0.union(&self.zeros.0).collect::<BitSet>())
    }

    pub fn control(&mut self, qubits: &QubitVec, condition: bool) {
        if condition {
            self.control_one(qubits);
        } else {
            self.control_zero(qubits);
        }
    }

    pub fn control_one(&mut self, qubits: &QubitVec) {
        self.ones.add(qubits);
    }

    pub fn control_zero(&mut self, qubits: &QubitVec) {
        self.zeros.add(qubits);
    }

    pub fn decontrol_one(&mut self, qubits: &QubitVec) {
        self.ones -= qubits.clone().into();
    }

    pub fn decontrol_zero(&mut self, qubits: &QubitVec) {
        self.zeros -= qubits.clone().into();
    }

    pub fn decontrol(&mut self, qubits: &QubitVec) {
        self.decontrol_one(qubits);
        self.decontrol_zero(qubits);
    }

    pub fn get(&self, qubit: QubitAddr) -> ControlState {
        match (self.ones.contains(qubit), self.zeros.contains(qubit)) {
            (false, false) => ControlState::None,
            (true, false) => ControlState::One,
            (false, true) => ControlState::Zero,
            (true, true) => ControlState::Both,
        }
    }

    pub fn contains(&self, qubit: QubitAddr) -> bool {
        self.ones.contains(qubit) || self.zeros.contains(qubit)
    }
}
