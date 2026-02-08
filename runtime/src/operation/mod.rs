pub mod elementary;
pub mod controlled;

use crate::gate::elementary::ElementaryGate;
use crate::gate::{DoubleTargetGate, DynamicTargetGate, SingleTargetGate, TripleTargetGate};
use crate::algebra::{Mat2, ToMat, ToMat2, ToMat4, ToMat8};
use crate::operation::controlled::ControlledOperation;
use crate::operation::elementary::ElementaryOperation;
use crate::qubit::qubit_accessor::QubitAccessor;
use crate::qubit::QubitAddr;

pub type TargetSingle = QubitAddr;
pub type TargetDouble = (QubitAddr, QubitAddr);
pub type TargetTriple = (QubitAddr, QubitAddr, QubitAddr);
pub type TargetMultiple = QubitAccessor;

pub trait ElementaryGateOperation {
    fn get_gate(&self) -> ElementaryGate;
    fn get_target(&self) -> QubitAccessor;
}

#[macro_export]
macro_rules! impl_get_gate {
    ($type:ty) => {
        impl ElementaryGateOperation for $type {
            fn get_gate(&self) -> ElementaryGate {
                self.gate.into()
            }
        }
    };
}

pub trait SingleTargetOperation: ToMat2 + Into<ElementaryOperation> {
    type Gate: SingleTargetGate;

    fn get_gate(&self) -> Self::Gate;
    fn get_target(&self) -> TargetSingle;

    fn gate_ident(&self) -> String {
        self.get_gate().ident()
    }
}

pub trait DoubleTargetOperation: ToMat4 + Into<ElementaryOperation> {
    type Gate: DoubleTargetGate;

    fn get_gate(&self) -> Self::Gate;
    fn get_target(&self) -> TargetDouble;

    fn gate_ident(&self) -> String {
        self.get_gate().ident()
    }
}

pub trait TripleTargetOperation: ToMat8 + Into<ElementaryOperation> {
    type Gate: TripleTargetGate;

    fn get_gate(&self) -> Self::Gate;
    fn get_target(&self) -> TargetTriple;

    fn gate_ident(&self) -> String {
        self.get_gate().ident()
    }
}

pub trait DynamicTargetOperation: ToMat {
    type Gate: DynamicTargetGate;
    fn get_gate(&self) -> Self::Gate;
    fn get_target(&self) -> TargetMultiple;
}

pub trait MultiTargetOperation: ToMat {
    fn get_target_accessor(&self) -> QubitAccessor;
}

#[macro_export]
macro_rules! impl_multi_target_op {
    ($type:ty : double) => {
        impl MultiTargetOperation for $type {
            fn get_target_accessor(&self) -> QubitAccessor {
                use $crate::qubits;
                let (t0, t1) = self.get_target();
                qubits![t0, t1]
            }
        }
    };
    ($type:ty : triple) => {
        impl MultiTargetOperation for $type {
            fn get_target_accessor(&self) -> QubitAccessor {
                use $crate::qubits;
                let (t0, t1, t2) = self.get_target();
                qubits![t0, t1, t2]
            }
        }
    };
    ($type:ty : triple) => {
        impl MultiTargetOperation for $type {
            fn get_target_accessor(&self) -> QubitAccessor {
                self.get_target()
            }
        }
    };
}

#[derive(Clone, Debug)]
pub enum Operation {
    Elementary(ElementaryOperation),
    Controlled(ControlledOperation),
}
