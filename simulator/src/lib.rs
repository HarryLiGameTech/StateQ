use std::os::raw::c_char;

#[repr(C)]
pub struct ExecuteResult {
    pub error_code: u8,
    pub measurement: MeasurementResult,
}

#[repr(C)]
pub struct MeasurementResult {
    pub shots: u64,
    pub result_size: u64,
    pub measurements: *mut MeasurementResultEntry,
}

#[repr(C)]
pub struct MeasurementResultEntry {
    pub value: u64,
    pub count: u64,
}

#[link(name = "qil")]
extern "C" {
    pub fn _qivm_available_qubits() -> u32;
    pub fn _qivm_is_gate_available(gate_ident: *const c_char) -> bool;
    pub fn _qivm_exec_bytecode(
        raw_bytecode: *const u8, bytecode_size: u32, qubits_alloc: u32
    ) -> ExecuteResult;
}

#[no_mangle]
pub unsafe extern fn qivm_available_qubits() -> u32 {
    unsafe { _qivm_available_qubits() }
}

#[no_mangle]
pub unsafe extern fn qivm_is_gate_available(gate_ident: *const c_char) -> bool {
    unsafe { _qivm_is_gate_available(gate_ident) }
}

#[no_mangle]
pub unsafe extern fn qivm_exec_bytecode(
    raw_bytecode: *const u8, bytecode_size: u32, qubits_alloc: u32
) -> ExecuteResult {
    unsafe { _qivm_exec_bytecode(raw_bytecode, bytecode_size, qubits_alloc) }
}
