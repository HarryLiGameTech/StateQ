use crate::gate::unitary::{UnitaryDoubleGate, UnitaryDynamicGate, UnitarySingleGate};
use crate::operation::{
    SingleTargetOperation, DoubleTargetOperation, TripleTargetOperation, DynamicTargetOperation,
    TargetSingle, TargetDouble, TargetTriple, TargetMultiple,
};

pub mod elementary;
pub mod standard;
pub mod unitary;
pub mod canonical;
pub mod rotation;
pub mod custom;

pub trait Dagger {
    fn dagger(self) -> Self;
}

pub trait IntoUnitary<U> {
    fn into_unitary(self) -> U;
}

pub trait SingleTargetGate: IntoUnitary<UnitarySingleGate> {
    type Operation: SingleTargetOperation;
    fn ident(&self) -> String;
    fn apply_to(self, target: TargetSingle) -> Self::Operation;
}

pub trait DoubleTargetGate: IntoUnitary<UnitaryDoubleGate> {
    type Operation: DoubleTargetOperation;
    fn ident(&self) -> String;
    fn apply_to(self, target: TargetDouble) -> Self::Operation;
}

pub trait TripleTargetGate {
    type Operation: TripleTargetOperation;
    fn ident(&self) -> String;
    fn apply_to(self, target: TargetTriple) -> Self::Operation;
}

pub trait DynamicTargetGate: IntoUnitary<UnitaryDynamicGate> {
    type Operation: DynamicTargetOperation;
    fn apply_to(self, target: TargetMultiple) -> Self::Operation;
}
