use std::fmt::Display;
use std::os::raw::c_char;
use crate::bytecode::ByteCode;
use crate::measurement::{MeasurementResult, RawMeasurementResult};

#[derive(Debug)]
pub struct ExecuteResult {
    pub error_code: u8,
    pub measurement: MeasurementResult,
}

#[repr(C)]
pub struct RawExecuteResult {
    pub error_code: u8,
    pub measurement: RawMeasurementResult,
}

impl From<RawExecuteResult> for ExecuteResult {
    fn from(raw: RawExecuteResult) -> Self {
        Self {
            error_code: raw.error_code,
            measurement: raw.measurement.into(),
        }
    }
}

impl From<ExecuteResult> for RawExecuteResult {
    fn from(result: ExecuteResult) -> Self {
        Self {
            error_code: result.error_code,
            measurement: result.measurement.into(),
        }
    }
}

/// Get the number of available qubits for the backend.
#[cfg(any(static_link_backend, dynamic_link_backend, test))]
pub fn get_available_qubits() -> usize {
    unsafe { qivm_available_qubits() as usize }
}

/// Get the number of available qubits for the backend.
#[cfg(not(any(static_link_backend, dynamic_link_backend, test)))]
pub fn get_available_qubits() -> usize {
    QIVM_AVAILABLE_QUBITS() as usize
}

/// Execute the compiled bytecode.
#[cfg(any(static_link_backend, dynamic_link_backend, test))]
pub fn execute_bytecode(bytecode: ByteCode, shots: usize) -> RawExecuteResult {
    unsafe {
        qivm_exec_bytecode(bytecode.as_ptr(), bytecode.len() as u32, shots as u32)
    }
}

/// Execute the compiled bytecode.
#[cfg(not(any(static_link_backend, dynamic_link_backend, test)))]
pub fn execute_bytecode(bytecode: ByteCode, exec_times: usize) -> RawExecuteResult {
    QIVM_EXEC_BYTECODE(bytecode.as_ptr(), bytecode.len() as u32, exec_times as u32)
}

/// Check if the gate is available in the backend.
#[cfg(any(static_link_backend, dynamic_link_backend, test))]
pub fn is_gate_available(gate_ident: &str) -> bool {
    let c_str = std::ffi::CString::new(gate_ident).unwrap();
    unsafe { qivm_is_gate_available(c_str.as_ptr()) }
}

/// Check if the gate is available in the backend.
#[cfg(not(any(static_link_backend, dynamic_link_backend, test)))]
pub fn is_gate_available(gate_ident: &str) -> bool {
    let c_str = std::ffi::CString::new(gate_ident).unwrap();
    QIVM_IS_GATE_AVAILABLE(c_str.as_ptr())
}

#[cfg(any(static_link_backend, dynamic_link_backend, test))]
#[link(name = "qil")]
extern "C" {
    fn qivm_available_qubits() -> u32;
    fn qivm_is_gate_available(gate_ident: *const c_char) -> bool;
    fn qivm_exec_bytecode(raw_bytecode: *const u8, bytecode_size: u32, shots: u32) -> RawExecuteResult;
}

type FnQivmAvailableQubits = libloading::Symbol<'static, fn() -> u32>;
type FnQivmIsGateAvailable = libloading::Symbol<'static, fn(*const c_char) -> bool>;
type FnQivmExecBytecode = libloading::Symbol<'static, fn(*const u8, u32, u32) -> RawExecuteResult>;

#[cfg(not(any(static_link_backend, dynamic_link_backend, test)))]
lazy_static! {
    static ref LIB_QIVM_BACKEND: libloading::Library = unsafe {
        libloading::Library::new("libqil.so").unwrap_or_else(|_| {
            raise_error!("Unable to load `libqil.so`")
        })
    };

    static ref QIVM_AVAILABLE_QUBITS: FnQivmAvailableQubits = unsafe {
        LIB_QIVM_BACKEND.get(b"qivm_available_gates").unwrap()
    };

    static ref QIVM_IS_GATE_AVAILABLE: FnQivmIsGateAvailable = unsafe {
        LIB_QIVM_BACKEND.get(b"qivm_is_gate_available").unwrap()
    };

    static ref QIVM_EXEC_BYTECODE: FnQivmExecBytecode = unsafe {
        LIB_QIVM_BACKEND.get(b"qivm_exec_bytecode").unwrap()
    };
}
