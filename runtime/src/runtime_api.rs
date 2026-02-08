use std::cell::RefCell;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr::slice_from_raw_parts;
use std::rc::Rc;
use std::slice;
use num::complex::Complex64;
use crate::program::QuantumProgramContext;
use crate::{QIVM_INSTANCE, raise_error};
use crate::backend::{execute_bytecode, ExecuteResult, RawExecuteResult};
use crate::gate::standard::{StandardGate, StandardSingleGate, StandardTripleGate};
use crate::gate::standard::StandardDoubleGate;
use crate::measurement::{MeasurementResult, MeasurementResultEntry, RawMeasurementResult};
use crate::program::builder::QuantumProgramContextBuilder;
use crate::qubit::qubit_accessor::QubitAccessor;
use crate::qubit::{QubitAddr, Slice};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RawComplex {
    pub re: f64,
    pub im: f64,
}

impl From<Complex64> for RawComplex {
    fn from(value: Complex64) -> Self {
        Self { re: value.re, im: value.im }
    }
}

impl From<RawComplex> for Complex64 {
    fn from(value: RawComplex) -> Self {
        Complex64::new(value.re, value.im)
    }
}

#[no_mangle]
pub extern fn qivm_get_program_ctx() -> *mut QuantumProgramContext {
    let mut ctx_builder = QuantumProgramContextBuilder::new();
    ctx_builder.default_passes();
    Box::into_raw(Box::new(ctx_builder.build()))
}

#[no_mangle]
pub unsafe extern fn qivm_destroy_program_ctx(ctx: *mut QuantumProgramContext) {
    // To destroy the value later, use `Box::from_raw` to create a new Box that owns it,
    // then let that box deallocate its contained value when it goes out of scope.
    let _ = Box::from_raw(ctx);
}

#[no_mangle]
pub unsafe extern fn qivm_measure(ctx: *mut QuantumProgramContext, accessor: *mut QubitAccessor) {
    let ctx = ctx.unsafe_into();
    let accessor = accessor.unsafe_into().clone();
    ctx.measure(accessor);
}

#[no_mangle]
pub unsafe extern fn qivm_exec_program(ctx: *mut QuantumProgramContext, shots: u64) -> u8 {
    let ctx = ctx.unsafe_into();
    let bytecode = ctx.compile_bytecode();
    let result: ExecuteResult = execute_bytecode(bytecode, shots as usize).into();
    ctx.set_measurement_result(result.measurement);
    result.error_code
}

#[no_mangle]
pub unsafe extern fn qivm_stack_enter(ctx: *mut QuantumProgramContext) {
    ctx.unsafe_into().enter();
}

#[no_mangle]
pub unsafe extern fn qivm_stack_exit(ctx: *mut QuantumProgramContext) {
    ctx.unsafe_into().exit();
}

#[no_mangle]
pub unsafe extern fn qivm_alloc_qubits(
    ctx: *mut QuantumProgramContext, size: u64
) -> *mut QubitAccessor {
    ctx.unsafe_into().alloc(size as usize).as_ptr()
}

#[no_mangle]
pub unsafe extern fn qivm_qubit_accessor_encode(
    ctx: *mut QuantumProgramContext, accessor: *mut QubitAccessor, value: u32
) {
    let ctx = ctx.unsafe_into();
    ctx.encode(accessor.unsafe_into(), value);
}

#[no_mangle]
pub unsafe extern fn qivm_qubit_accessor_size(accessor: *mut QubitAccessor) -> u64 {
    accessor.unsafe_into().size() as u64
}

#[no_mangle]
pub unsafe extern fn qivm_qubit_accessor_concat(
    ctx: *mut QuantumProgramContext, lhs: *mut QubitAccessor, rhs: *mut QubitAccessor,
) -> *mut QubitAccessor {
    let result = Rc::new(RefCell::new(
        lhs.unsafe_into().clone() + rhs.unsafe_into().clone()
    ));
    ctx.unsafe_into().add_qubit_accessor(result.clone());
    result.as_ptr()
}

#[no_mangle]
pub unsafe extern fn qivm_qubit_accessor_indexing(
    ctx: *mut QuantumProgramContext, accessor: *mut QubitAccessor, index: i64
) -> *mut QubitAccessor {
    let accessor = accessor.unsafe_into();
    let index = if index < 0 { accessor.size() as i64 + index } else { index } as usize;
    let result = Rc::new(RefCell::new(accessor.get(index)));
    ctx.unsafe_into().add_qubit_accessor(result.clone());
    result.as_ptr()
}

#[no_mangle]
pub unsafe extern fn qivm_qubit_accessor_slicing(
    ctx: *mut QuantumProgramContext, accessor: *mut QubitAccessor,
    from: i32, to: i32, step: u64,
) -> *mut QubitAccessor {
    let accessor = accessor.unsafe_into();
    let size = accessor.size() as i32;
    let result = Rc::new(RefCell::new(
        accessor.slice(
            if from < 0 { size + from } else { from } as QubitAddr,
            if to < 0 { size + to } else { to } as QubitAddr,
            step as usize
        )
    ));
    ctx.unsafe_into().add_qubit_accessor(result.clone());
    result.as_ptr()
}

#[no_mangle]
pub unsafe extern fn qivm_program_begin_ctrl(
    ctx: *mut QuantumProgramContext,
    ctrl_qubits: *const QubitAccessor,
    condition: bool,
) {
    let ctx: &mut QuantumProgramContext = ctx.unsafe_into();
    ctx.control(unsafe { ctrl_qubits.as_ref().unwrap().clone() }, condition);
}

#[no_mangle]
pub unsafe extern fn qivm_program_end_ctrl(
    ctx: *mut QuantumProgramContext,
    ctrl_qubits: *const QubitAccessor
) {
    let ctx: &mut QuantumProgramContext = ctx.unsafe_into();
    ctx.decontrol(unsafe { ctrl_qubits.as_ref().unwrap().clone() });
}

#[no_mangle]
pub unsafe extern fn qivm_program_begin_dagger(ctx: *mut QuantumProgramContext) {
    let ctx: &mut QuantumProgramContext = ctx.unsafe_into();
    ctx.begin_dagger();
}

#[no_mangle]
pub unsafe extern fn qivm_program_end_dagger(ctx: *mut QuantumProgramContext) {
    let ctx: &mut QuantumProgramContext = ctx.unsafe_into();
    ctx.end_dagger();
}

#[no_mangle]
pub unsafe extern fn qivm_program_pause_ctrl(ctx: *mut QuantumProgramContext) {
    let ctx: &mut QuantumProgramContext = ctx.unsafe_into();
    ctx.pause_ctrl();
}

#[no_mangle]
pub unsafe extern fn qivm_program_restore_ctrl(ctx: *mut QuantumProgramContext) {
    let ctx: &mut QuantumProgramContext = ctx.unsafe_into();
    ctx.restore_ctrl();
}

fn reinterpret_cast<T, U>(value: T) -> U {
    unsafe { std::mem::transmute_copy(&value) }
}

#[no_mangle]
pub unsafe extern fn qivm_program_push_op(
    ctx: *mut QuantumProgramContext, ident: *const c_char,
    target_qubits: *const QubitAccessor,
    params: *const u64, param_size: u64,
) {
    let ctx: &mut QuantumProgramContext = ctx.unsafe_into();
    let ident = CStr::from_ptr(ident).to_str().unwrap();
    let params = std::slice::from_raw_parts(params, param_size as usize);
    let gate: StandardGate = match ident {
        "H" => StandardSingleGate::H.into(),
        "X" => StandardSingleGate::X.into(),
        "Y" => StandardSingleGate::Y.into(),
        "Z" => StandardSingleGate::Z.into(),
        "S" => StandardSingleGate::S.into(),
        "T" => StandardSingleGate::T.into(),
        "P" => StandardSingleGate::P { angle: reinterpret_cast(params[0]) }.into(),
        "RX" => StandardSingleGate::RX { angle: reinterpret_cast(params[0]) }.into(),
        "RY" => StandardSingleGate::RY { angle: reinterpret_cast(params[0]) }.into(),
        "RZ" => StandardSingleGate::RZ { angle: reinterpret_cast(params[0]) }.into(),
        "CX" => StandardDoubleGate::CX.into(),
        "CZ" => StandardDoubleGate::CZ.into(),
        "SWP" => StandardDoubleGate::SWP.into(),
        "CP" => StandardDoubleGate::CP { angle: reinterpret_cast(params[0]) }.into(),
        "CCX" => StandardTripleGate::CCX.into(),
        _ => raise_error!("Unsupported standard gate: {}", ident),
    };
    ctx.push(gate, unsafe { target_qubits.as_ref().unwrap().clone() });
}

#[no_mangle]
pub unsafe extern fn qivm_program_push_custom_op(
    ctx: *mut QuantumProgramContext, ident: *const c_char,
    target_size: u64, mat: *const RawComplex,
    target_qubits: *const QubitAccessor,
    param_size: u64, params: *const u64,
) {
    let ctx: &mut QuantumProgramContext = ctx.unsafe_into();
    let ident = unsafe {
        CStr::from_ptr(ident).to_str().unwrap()
    }.to_string();
    let mat_slice: Vec<Complex64> = unsafe {
        slice::from_raw_parts(mat, 2usize.pow(target_size as u32 * 2))
    }.iter().copied().map(Into::<Complex64>::into).collect();
    let params: Vec<u64> = unsafe {
        slice::from_raw_parts(params, param_size as usize)
    }.to_vec();
    let target_qubits = unsafe { target_qubits.as_ref().unwrap().clone() };
    ctx.push_custom(ident, mat_slice.as_slice().into(), params, target_qubits);
}

#[no_mangle]
pub unsafe extern fn qivm_program_push_custom_builtin_op(
    ctx: *mut QuantumProgramContext, ident: *const c_char,
    target_size: u64, target_qubits: *const QubitAccessor,
    param_size: u64, params: *const u64,
) {
    let ctx: &mut QuantumProgramContext = ctx.unsafe_into();
    let ident = unsafe {
        CStr::from_ptr(ident).to_str().unwrap()
    }.to_string();
    let params: Vec<u64> = unsafe {
        slice::from_raw_parts(params, param_size as usize)
    }.to_vec();
    let target_qubits = unsafe { target_qubits.as_ref().unwrap().clone() };
    ctx.push_custom_builtin(ident, params, target_size as usize, target_qubits);
}

#[no_mangle]
pub unsafe extern fn qivm_program_get_result(ctx: *mut QuantumProgramContext) -> RawMeasurementResult {
    let ctx: &mut QuantumProgramContext = ctx.unsafe_into();
    let result = ctx.get_measurement_result();
    result.unwrap_or_else(|| {
        raise_error!("Measurement result is not available");
    }).into_raw()
}

#[no_mangle]
pub unsafe extern fn qivm_program_assign_result(
    ctx: *mut QuantumProgramContext, result: *mut RawMeasurementResult,
) -> u64 {
    let ctx: &mut QuantumProgramContext = ctx.unsafe_into();
    let raw_result: &mut RawMeasurementResult = result.unsafe_into();
    let result = ctx.get_measurement_result().unwrap_or_else(|| {
        raise_error!("Measurement result is not available");
    });
    raw_result.shots = result.shots;
    if result.measurements.len() > raw_result.result_size as usize {
        raise_error!("Measurement result buffer is too small");
    }
    let raw_measurements = slice::from_raw_parts_mut(raw_result.measurements, raw_result.result_size as usize);
    result.measurements.iter().enumerate().for_each(|(i, &v)| {
        raw_measurements[i] = v;
    });
    raw_result.result_size = result.measurements.len() as u64;
    result.measurements.len() as u64
}

unsafe impl<'a> UnsafeInto<&'a mut QuantumProgramContext> for *mut QuantumProgramContext {
    unsafe fn unsafe_into(self) -> &'a mut QuantumProgramContext {
        self.as_mut().unwrap_or_else(|| {
            raise_error!("Invalid quantum program context")
        })
    }
}

unsafe impl<'a> UnsafeInto<&'a mut QubitAccessor> for *mut QubitAccessor {
    unsafe fn unsafe_into(self) -> &'a mut QubitAccessor {
        self.as_mut().unwrap_or_else(|| {
            raise_error!("Invalid qubit accessor")
        })
    }
}

unsafe impl<'a> UnsafeInto<&'a mut RawMeasurementResult> for *mut RawMeasurementResult {
    unsafe fn unsafe_into(self) -> &'a mut RawMeasurementResult {
        self.as_mut().unwrap_or_else(|| {
            raise_error!("Invalid measurement result")
        })
    }
}

unsafe trait UnsafeInto<T>: Sized {
    unsafe fn unsafe_into(self) -> T;
}
