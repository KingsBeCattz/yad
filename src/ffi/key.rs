use std::ffi::{c_char, CStr};
use crate::core::{Key, Value};

/// Frees a heap-allocated `Key`.
///
/// # Parameters
/// - `val`: Pointer to the `Key` to free.
///
/// # Safety
/// - `val` must be a pointer previously returned from FFI or null.
/// - After calling this function, `val` must not be used again.
#[unsafe(no_mangle)]
pub extern "C" fn key_free(val: *mut Key) {
    if !val.is_null() {
        unsafe { let _ = Box::from_raw(val); }
    }
}

/// Creates a new heap-allocated `Key` with a given name and value.
///
/// # Parameters
/// - `c_name`: Pointer to a null-terminated C string containing the key name.
/// - `val`: Pointer to a heap-allocated `Value` to assign to the key.
///
/// # Returns
/// - Pointer to a heap-allocated `Key`.
/// - Returns `null` if any pointer is null or if the name conversion fails.
///
/// # Safety
/// - Ownership of `val` is transferred to the `Key`.
/// - The returned pointer must eventually be freed with `key_free` to avoid memory leaks.
#[unsafe(no_mangle)]
pub extern "C" fn key_new(c_name: *const c_char, val: *mut Value) -> *mut Key {
    if c_name.is_null() || val.is_null() {
        return std::ptr::null_mut()
    }

    let name = unsafe { CStr::from_ptr(c_name) };
    let name = match name.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    unsafe {
        let value: Box<Value> = Box::from_raw(val);
        Box::into_raw(Box::new(Key::new(name.to_string(), *value)))
    }
}
