
pub mod qubit_set;
pub mod qubit_vec;
pub mod ctrl_qubit_set;

#[cfg(test)]
mod tests;

pub type QubitAddr = u32;

pub trait Slice<T: Sized> {
    fn slice(&self, from: QubitAddr, to: QubitAddr, step: usize) -> T;
}
