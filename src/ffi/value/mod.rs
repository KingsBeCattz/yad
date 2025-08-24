use crate::core::Value;

/// Submodules providing specialized functions for different types of `Value`.
///
/// Each module handles operations specific to the type, for example:
/// - `_8b_functions` : Functions for 8-bit values
/// - `_16b_functions`: Functions for 16-bit values
/// - `_32b_functions`: Functions for 32-bit values
/// - `_64b_functions`: Functions for 64-bit values
/// - `_bool_functions`: Functions for boolean values
/// - `_string_functions`: Functions for string values
/// - `_array_functions`: Functions for array values
mod _8b_functions;
pub use _8b_functions::*;
mod _32b_functions;
pub use _32b_functions::*;
mod _16b_functions;
pub use _16b_functions::*;
mod _64b_functions;
pub use _64b_functions::*;
mod _bool_functions;
pub use _bool_functions::*;
mod _string_functions;
pub use _string_functions::*;
mod _array_functions;
pub use _array_functions::*;

/// Frees a `Value` previously allocated on the heap.
///
/// # Parameters
/// - `val`: Pointer to a `Value`.
///
/// # Safety
/// - `val` must be a pointer previously returned from a creation function.
/// - After calling this function, the pointer must not be used again.
#[unsafe(no_mangle)]
pub extern "C" fn value_free(val: *mut Value) {
    if !val.is_null() {
        unsafe { drop(Box::from_raw(val)) }
    }
}

/// Returns the type of the `Value`.
///
/// # Parameters
/// - `val`: Pointer to a `Value`.
///
/// # Returns
/// - A `u8` representing the type.
/// - Returns `0` if `val` is null.
///
/// # Safety
/// - `val` must be a valid pointer or null.
#[unsafe(no_mangle)]
pub extern "C" fn value_type(val: *mut Value) -> u8 {
    if val.is_null() {
        return 0
    }

    unsafe {
        (*val).r#type as u8
    }
}

/// Returns the length of the `Value` in bytes (or its logical length).
///
/// # Parameters
/// - `val`: Pointer to a `Value`.
///
/// # Returns
/// - Length as `u8`.
/// - Returns `0` if `val` is null.
///
/// # Safety
/// - `val` must be a valid pointer or null.
#[unsafe(no_mangle)]
pub extern "C" fn value_len(val: *mut Value) -> u8 {
    if val.is_null() {
        return 0
    }

    unsafe {
        (&*val).length as u8
    }
}

/// Returns a raw pointer to the underlying bytes of the `Value`.
///
/// # Parameters
/// - `val`: Pointer to a `Value`.
///
/// # Returns
/// - Pointer to the first byte of the value, or null if `val` is null.
///
/// # Safety
/// - The returned pointer is valid as long as the `Value` is alive.
/// - Modifying the memory through this pointer may cause undefined behavior.
#[unsafe(no_mangle)]
pub extern "C" fn value_raw_bytes(val: *mut Value) -> *const u8 {
    if val.is_null() {
        return std::ptr::null()
    }

    unsafe {
        (&*val).bytes.as_ptr()
    }
}

/// Returns the length of the raw byte buffer of the `Value`.
///
/// # Parameters
/// - `val`: Pointer to a `Value`.
///
/// # Returns
/// - Number of bytes in the `Value`, or `0` if `val` is null.
///
/// # Safety
/// - `val` must be a valid pointer or null.
#[unsafe(no_mangle)]
pub extern "C" fn value_raw_bytes_length(val: *mut Value) -> usize {
    if val.is_null() {
        return 0
    }

    unsafe {
        (&*val).bytes.len()
    }
}
