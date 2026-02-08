use crate::gate::elementary::{ElementaryGate, SingleGate};
use crate::gate::rotation::Rotation::{Rx, Ry, Rz};
use crate::gate::standard::{StandardGate, StandardSingleGate};

/// Rotation around the x, y, or z axis.
#[derive(Copy, Clone, Debug)]
pub enum Rotation {
    Rx(f64),
    Ry(f64),
    Rz(f64)
}

/// Convert a rotation into a standard Rx/Ry/Rz gate.
impl Into<StandardSingleGate> for Rotation {
    fn into(self) -> StandardSingleGate {
        match self {
            Rx(angle) => StandardSingleGate::RX { angle },
            Ry(angle) => StandardSingleGate::RY { angle },
            Rz(angle) => StandardSingleGate::RZ { angle },
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<StandardGate> for Rotation {
    fn into(self) -> StandardGate {
        Into::<StandardSingleGate>::into(self).into()
    }
}

#[allow(clippy::from_over_into)]
impl Into<ElementaryGate> for Rotation {
    fn into(self) -> ElementaryGate {
        Into::<StandardGate>::into(self).into()
    }
}

#[allow(clippy::from_over_into)]
impl Into<SingleGate> for Rotation {
    fn into(self) -> SingleGate {
        Into::<StandardSingleGate>::into(self).into()
    }
}
