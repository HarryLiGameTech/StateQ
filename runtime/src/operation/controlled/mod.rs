use crate::operation::controlled::cond_ctrl::ConditionalCtrlOperation;
use crate::operation::controlled::mux::MuxOperation;

pub mod single_ctrl;
pub mod cond_ctrl;
pub mod multi_ctrl;
pub mod mux;

#[cfg(test)]
mod test;

#[derive(Clone, Debug)]
pub enum ControlledOperation {
    Mux(MuxOperation),
    ConditionalCtrl(ConditionalCtrlOperation),
}
