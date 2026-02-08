pub mod single_target;
pub mod multi_target;
pub mod rotation;

use crate::gate::Dagger;
use crate::dispatch;
use crate::operation::controlled::mux::multi_target::MultiTargetMuxOperation;
use crate::operation::controlled::mux::rotation::MuxRotationOperation;
use crate::operation::controlled::mux::single_target::SingleTargetMuxOperation;

#[derive(Clone, Debug)]
pub enum MuxOperation {
    SingleTarget(SingleTargetMuxOperation),
    MultiTarget(MultiTargetMuxOperation),
    Rotation(MuxRotationOperation),
}

impl Dagger for MuxOperation {
    fn dagger(self) -> Self {
        use MuxOperation::*;
        dispatch!(self; MultiTarget | Rotation => |gate| gate.dagger().into())
    }
}
