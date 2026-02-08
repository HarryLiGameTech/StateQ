pub mod elementary;
pub mod multiplexed;
mod multi_ctrl;

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Pointer};
use enum_dispatch::enum_dispatch;
use nalgebra::{DefaultAllocator, DimName, DimSub, Dynamic, U2, U4, U8};
use num::complex::Complex64;
use num_traits::Pow;
use crate::algebra::{ToDMat, DMat, ToMat};
use crate::circuit::Circuit;
use crate::operation::elementary::ElementaryGate;
use crate::operation::elementary::standard::StandardGate;
use crate::operation::elementary::unitary::UnitaryGate;
use crate::operation::multi_ctrl::MultiCtrlGate;
use crate::operation::multiplexed::MultiplexedOperation;
use crate::qubit::qubit_vec::QubitVec;
use crate::qubit::QubitAddr;
use crate::operation::elementary::standard::*;

type DimGate1 = U2;
type DimGate2 = U4;
type DimGate3 = U8;

pub trait Ident {
    fn ident(&self) -> &'static str;
}

#[enum_dispatch(QuantumOperation, ElementaryGate, StandardGate)]
pub trait Targets {
    fn targets(&self) -> QubitVec;
}

#[enum_dispatch(QuantumOperation, ElementaryGate, StandardGate)]
pub trait Operation: ToDMat + Targets {
    fn map_qubits(&self, f: &dyn Fn(QubitAddr) -> QubitAddr) -> Self where Self: Sized;
    fn size(&self) -> usize;
}

pub trait Dagger<G> {
    fn dagger(self) -> G;
}

pub trait StaticOperation<D: DimName>: Operation + ToMat<D>
where DefaultAllocator: nalgebra::allocator::Allocator<Complex64, D, D> {}

pub trait SingleOperation: StaticOperation<DimGate1> {
    fn target(&self) -> QubitAddr;
}

#[macro_export]
macro_rules! impl_single_gate {
    ($type:ty : $ident:literal) => {
        impl ToDMat for $type {
            fn to_dyn_mat(&self) -> DMat {
                let mat = self.to_mat();
                let n = mat.nrows();
                DMat::from_row_slice(n, n, mat.as_slice())
            }
        }

        impl Targets for $type {
            fn targets(&self) -> QubitVec {
                QubitVec::from(self.target)
            }
        }
    };
}

pub trait DoubleOperation: StaticOperation<DimGate2> {
    fn target0(&self) -> QubitAddr;
    fn target1(&self) -> QubitAddr;
}

#[macro_export]
macro_rules! impl_double_gate {
    ($type:ty : $ident:literal) => {
        impl ConstIdent for $type {
            fn ident() -> &'static str {
                $ident
            }
        }

        impl ToDMat for $type {
            fn to_dyn_mat(&self) -> DMat {
                let mat = self.to_mat();
                let n = mat.nrows();
                DMat::from_row_slice(n, n, mat.as_slice())
            }
        }

        impl Targets for $type {
            fn targets(&self) -> QubitVec {
                vec![self.target0(), self.target1()]
            }
        }
    };
}

pub trait TripleOperation: StaticOperation<DimGate3> {
    fn target0(&self) -> QubitAddr;
    fn target1(&self) -> QubitAddr;
    fn target2(&self) -> QubitAddr;
}

#[macro_export]
macro_rules! impl_triple_gate {
    ($type:ty : $ident:literal) => {
        impl ConstIdent for $type {
            fn ident() -> &'static str {
                $ident
            }
        }

        impl ToDMat for $type {
            fn to_dyn_mat(&self) -> DMat {
                let mat = self.to_mat();
                let n = mat.nrows();
                DMat::from_row_slice(n, n, mat.as_slice())
            }
        }

        impl Targets for $type {
            fn targets(&self) -> QubitVec {
                vec![self.target0(), self.target1(), self.target2()]
            }
        }
    };
}

pub trait Decompose<T: Operation> {
    fn decompose(&self) -> Circuit<T>;
}

#[derive(Debug)]
pub struct DecompositionError {
    message: String,
}

impl Display for DecompositionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for DecompositionError {}

#[enum_dispatch]
pub enum QuantumOperation {
    ElementaryGate,
    MultiCtrlGate(MultiCtrlGate<ElementaryGate>),
    MultiplexedOperation(MultiplexedOperation<ElementaryGate>),
}
