use enum_dispatch::enum_dispatch;

pub mod standard;
pub mod unitary;

use standard::StandardGate;
use unitary::UnitaryGate;
use crate::operation::ToDMat;
use crate::algebra::DMat;

#[enum_dispatch]
pub enum ElementaryGate {
    StandardGate,
    UnitaryGate,
}
