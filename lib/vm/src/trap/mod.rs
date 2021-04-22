// This file contains code from external sources.
// Attributions: https://github.com/wasmerio/wasmer/blob/master/ATTRIBUTIONS.md

//! This is the module that facilitates the usage of Traps
//! in Wasmer Runtime
mod trapcode;
mod traphandlers;

pub use trapcode::TrapCode;
pub use traphandlers::{
    CallThreadState, catch_traps, catch_traps_with_result, raise_lib_trap, raise_user_trap,
    wasmer_call_trampoline, take_tls, restore_tls, Trap,
};
pub use traphandlers::{init_traps, resume_panic};
