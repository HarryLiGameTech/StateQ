use std::ops::{Add, AddAssign, Index};
use std::slice::Iter;
use std::vec::IntoIter;
use crate::qubit::qubit_set::QubitSet;
use crate::qubit::{QubitAddr, Slice};
use crate::raise_error;

/// An ordered set to store qubit address
#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct QubitVec {
    /// TODO: use `usize` (single), `RangeInclusive<usize>` (range),
    /// `BTreeSet<RangeInclusive<usize>>` (ranges) and `Vec<usize>`
    /// as internal data structure to handle different situations
    /// more efficiently
    qubits: Vec<QubitAddr>,
}

impl QubitVec {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn range(from: QubitAddr, to: QubitAddr) -> Self {
        assert!(from <= to);
        Self { qubits: (from ..= to).collect() }
    }

    pub fn single(index: QubitAddr) -> Self {
        Self { qubits: vec![index] }
    }

    pub fn size(&self) -> usize {
        self.qubits.len()
    }

    pub fn is_empty(&self) -> bool {
        self.qubits.is_empty()
    }

    pub fn remove_index(&mut self, index: usize) -> QubitAddr {
        self.qubits.remove(index)
    }

    pub fn remove_qubit(&mut self, qubit: QubitAddr) -> bool {
        let prev_size = self.qubits.len();
        self.qubits.retain(|&x| x != qubit);
        prev_size == self.qubits.len()
    }

    pub fn pop(&mut self) -> QubitAddr {
        self.qubits.pop().unwrap_or_else(|| {
            raise_error!("Empty qubit accessor");
        })
    }

    pub fn insert(&mut self, index: QubitAddr) {
        if self.qubits.contains(&index) {
            raise_error!("Invalid qubit index: {}", index)
        }
        self.qubits.push(index)
    }

    pub fn insert_range(&mut self, from: QubitAddr, to: QubitAddr) {
        for qubit in &self.qubits {
            if from <= *qubit && *qubit <= to {
                raise_error!("Invalid qubit range: [{}, {}]", from, to);
            }
        }
        self.qubits.append(&mut Self::range(from, to).qubits)
    }

    pub fn to_vec(&self) -> Vec<QubitAddr> {
        self.qubits.clone()
    }

    pub fn iter(&self) -> Iter<QubitAddr> {
        self.qubits.iter()
    }

    pub fn into_iter(self) -> IntoIter<QubitAddr> {
        self.qubits.into_iter()
    }

    pub fn first(&self) -> QubitAddr {
        self.qubits[0]
    }

    pub fn get(&self, index: usize) -> QubitVec {
        QubitVec::single(*self.qubits.get(index).unwrap_or_else(|| {
            raise_error!("Invalid qubit index")
        }))
    }
}

#[macro_export]
macro_rules! qubits {
    ($($values:expr),* $(,)?) => {
        {
            use $crate::qubit::qubit_vec:QubitAccessorr;
            use $crate::qubit::QubitAddr;
            QubitVec::from(vec![$($values as QubitAddr),*])
        }
    };
}

impl Index<usize> for QubitVec {
    type Output = QubitAddr;

    fn index(&self, index: usize) -> &Self::Output {
        self.qubits.index(index)
    }
}

impl Slice<QubitVec> for QubitVec {
    fn slice(&self, from: QubitAddr, to: QubitAddr, step: usize) -> QubitVec {
        QubitVec::from(self.qubits.iter()
            .take(to as usize + 1)
            .skip(from as usize)
            .step_by(step).copied()
            .collect::<Vec<QubitAddr>>()
        )
    }
}

impl<T: AsRef<[QubitAddr]>> From<T> for QubitVec {
    fn from(iterable: T) -> Self {
        let mut qubits = Vec::new();
        iterable.as_ref().iter().for_each(|&e| { qubits.push(e); });
        Self { qubits }
    }
}

impl Add for QubitVec {
    type Output = QubitVec;

    fn add(self, rhs: Self) -> Self::Output {
        QubitVec {
            qubits: self.qubits.into_iter().chain(rhs.qubits).collect()
        }
    }
}

impl AddAssign for QubitVec {
    fn add_assign(&mut self, mut rhs: Self) {
        self.qubits.append(&mut rhs.qubits)
    }
}

impl From<QubitSet> for QubitVec {
    fn from(qubits: QubitSet) -> Self {
        Self::from(qubits.to_vec())
    }
}
