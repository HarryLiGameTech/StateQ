use std::ops::{Add, AddAssign, Sub, SubAssign};
use bit_set::BitSet;
use crate::qubit::QubitAddr;
use crate::qubit::qubit_accessor::QubitAccessor;

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct QubitSet(pub(super) BitSet<QubitAddr>);

impl QubitSet {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, accessor: &QubitAccessor) {
        accessor.iter().for_each(|&qubit| {
            self.0.insert(qubit as usize);
        })
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn remove(&mut self, index: usize) -> bool {
        self.0.remove(index)
    }

    pub fn pop(&mut self) -> Option<QubitAddr> {
        self.0.iter().next().map(|qubit| {
            self.remove(qubit);
            qubit as QubitAddr
        })
    }
    
    pub fn first(&self) -> Option<QubitAddr> {
        self.0.iter().next().map(|qubit| qubit as QubitAddr)
    }
    
    pub fn intersect_with(&mut self, rhs: &Self) {
        self.0.intersect_with(&rhs.0);
    }

    pub fn insert(&mut self, index: usize) -> bool {
        self.0.insert(index)
    }

    pub fn insert_range(&mut self, from: QubitAddr, to: QubitAddr) {
        self.0.union_with(&(from as usize ..= to as usize).collect())
    }

    pub fn to_vec(&self) -> Vec<QubitAddr> {
        self.0.iter().map(|qubit| qubit as QubitAddr).collect()
    }

    pub fn contains(&self, qubit: QubitAddr) -> bool {
        self.0.contains(qubit as usize)
    }
}

impl Add for QubitSet {
    type Output = QubitSet;

    fn add(self, rhs: Self) -> Self::Output {
        QubitSet(self.0.union(&rhs.0).collect())
    }
}

impl AddAssign for QubitSet {
    fn add_assign(&mut self, rhs: Self) {
        self.0.union_with(&rhs.0)
    }
}

impl Sub for QubitSet {
    type Output = QubitSet;

    fn sub(self, rhs: Self) -> Self::Output {
        QubitSet(self.0.difference(&rhs.0).collect())
    }
}

impl SubAssign for QubitSet {
    fn sub_assign(&mut self, rhs: Self) {
        self.0.difference_with(&rhs.0);
    }
}

impl From<QubitAccessor> for QubitSet {
    fn from(accessor: QubitAccessor) -> Self {
        Self(BitSet::from_iter(
            accessor.into_iter().map(|qubit| qubit as usize)
        ))
    }
}

impl From<Vec<QubitAddr>> for QubitSet {
    fn from(vec: Vec<QubitAddr>) -> Self {
        Self(BitSet::from_iter(vec.into_iter().map(|qubit| qubit as usize)))
    }
}
