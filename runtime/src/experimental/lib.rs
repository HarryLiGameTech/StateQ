#![allow(mixed_script_confusables)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::from_over_into)]
#![allow(clippy::missing_safety_doc)]
#![feature(generic_const_exprs)]

extern crate core;
extern crate proc_macro;

pub mod qubit;
mod algebra;
mod macros;
pub mod operation;
pub mod util;
pub mod circuit;
