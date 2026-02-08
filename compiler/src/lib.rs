use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::ops::Deref;
use std::os::raw::c_char;
use std::path::PathBuf;
use std::slice;

pub struct CompileResult {
    pub targets: Vec<PathBuf>,
    pub errors: Vec<CompileError>,
}

pub fn compile(source_path: &str, config: &BTreeMap<String, String>) -> CompileResult {
    let config = config.iter().map(|(k, v)| {
        RawKeyValueEntry::from((k.as_str(), v.as_str()))
    }).collect::<Vec<RawKeyValueEntry>>();
    unsafe {
        (*stateq_compile(
            CString::new(source_path).unwrap().into_raw(),
            &RawVec::from(config),
        )).into()
    }
}

unsafe fn c_char_ptr_to_string(ptr: *const c_char) -> String {
    CStr::from_ptr(ptr).to_str().unwrap().to_string()
}

pub struct CompileError {
    pub err_type: CompileErrType,
    pub source: String,
    pub line: i32,
    pub column: i32,
    pub message: String,
}

#[derive(Copy, Clone)]
pub enum CompileErrType {
    Error,
    Warning,
    Note,
    Help,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RawVec<T> {
    size: u32,
    data: *const T,
}

impl<T> Deref for RawVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self.data, self.size as usize)
        }
    }
}

impl<T> From<Vec<T>> for RawVec<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut vec = vec.into_boxed_slice();
        let data = vec.as_mut_ptr();
        let size = vec.len() as u32;
        std::mem::forget(vec);
        Self { size, data }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RawCompileError {
    err_type: i32,
    source: *const c_char,
    line: i32,
    column: i32,
    message: *const c_char,
}

impl From<RawCompileError> for CompileError {
    fn from(raw_err: RawCompileError) -> Self {
        unsafe {
            CompileError {
                err_type: match raw_err.err_type {
                    0 => CompileErrType::Error,
                    1 => CompileErrType::Warning,
                    2 => CompileErrType::Note,
                    3 => CompileErrType::Help,
                    _ => unreachable!()
                },
                source: c_char_ptr_to_string(raw_err.source),
                line: raw_err.line,
                column: raw_err.column,
                message: c_char_ptr_to_string(raw_err.message),
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RawCompileResult {
    pub target_files: RawVec<*const c_char>,
    pub errors: RawVec<RawCompileError>,
}

impl From<RawCompileResult> for CompileResult {
    fn from(raw: RawCompileResult) -> Self {
        Self {
            errors: (*raw.errors).iter().map(|err| (*err).into()).collect(),
            targets: (*raw.target_files).iter().map(|path_str| unsafe {
                PathBuf::from(&c_char_ptr_to_string(*path_str))
            }).collect(),
        }
    }
}

#[repr(C)]
struct RawKeyValueEntry {
    key: *const c_char,
    value: *const c_char,
}

impl From<(&str, &str)> for RawKeyValueEntry {
    fn from((key, value): (&str, &str)) -> Self {
        Self {
            key: CString::new(key).unwrap().into_raw(),
            value: CString::new(value).unwrap().into_raw(),
        }
    }
}

#[link(name = "stateq")]
extern "C" {
    fn stateq_compile(
        source_path: *const c_char, config: *const RawVec<RawKeyValueEntry>
    ) -> *const RawCompileResult;
}
