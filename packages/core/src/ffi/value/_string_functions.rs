use std::ffi::{c_char, CStr, CString};
use crate::core::Value;

/// Creates a heap-allocated `Value` from a C string (`*const c_char`).
///
/// # Parameters
/// - `c_string`: Pointer to a null-terminated C string.
///
/// # Returns
/// - Pointer to a heap-allocated `Value` containing the string.
/// - Returns `null` if the input pointer is null, or if the conversion fails.
///
/// # Safety
/// - `c_string` must point to a valid null-terminated C string or be null.
/// - The returned pointer must eventually be freed with `value_free` to avoid memory leaks.
#[unsafe(no_mangle)]
pub extern "C" fn value_from_cstring(c_string: *const c_char) -> *mut Value {
    if c_string.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(c_string) };
    let c_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    if let Ok(yad_string) = Value::from_string(c_str.to_string()) {
        Box::into_raw(Box::new(yad_string))
    } else {
        std::ptr::null_mut()
    }
}

/// Converts a `Value` containing a string into a C string (`*const c_char`).
///
/// # Parameters
/// - `value`: Pointer to a `Value` containing a string.
///
/// # Returns
/// - Pointer to a null-terminated C string allocated on the heap (`*const c_char`).
/// - Returns null if `value` is null or the conversion fails.
///
/// # Safety
/// - The returned C string is heap-allocated and must be freed using [`cstring_free`]
///   when no longer needed to avoid memory leaks.
#[unsafe(no_mangle)]
pub extern "C" fn cstring_from_value(value: *mut Value) -> *const c_char {
    if value.is_null() {
        return std::ptr::null();
    }

    unsafe {
        match (&*value).as_string() {
            Ok(string) => match CString::new(string) {
                Ok(cstr) => cstr.into_raw() as *const c_char,
                Err(_) => std::ptr::null(),
            },
            Err(_) => std::ptr::null(),
        }
    }
}

/// Frees a C string previously allocated by [`cstring_from_value`].
///
/// # Parameters
/// - `cstr`: Pointer to a C string allocated on the heap by [`cstring_from_value`].
///
/// # Safety
/// - `cstr` must be a valid pointer returned by [`cstring_from_value`].
/// - After calling this function, `cstr` must not be used again.
#[unsafe(no_mangle)]
pub extern "C" fn cstring_free(cstr: *mut c_char) {
    if cstr.is_null() {
        return;
    }

    unsafe {
        // Reconstruct CString to drop it and free memory
        drop(CString::from_raw(cstr))
    }
}

/// Returns the length of a string contained within a `Value` as a C-compatible string (cstring).
///
/// This function is intended to be called from external C code. It safely checks for null pointers
/// and ensures that the value can be converted to a string before returning its length.
///
/// # Safety
/// This function dereferences a raw pointer. The caller must ensure:
/// - `value` is either a valid pointer to a `Value` or null.
/// - The memory pointed to by `value` remains valid for the duration of this call.
///
/// # Parameters
/// - `value`: A mutable pointer to a `Value` instance. Can be null.
///
/// # Returns
/// - `usize`: The length of the string contained in the `Value`.
///   - Returns `0` if `value` is null or if the value cannot be converted to a string.
///
/// # Example (Rust)
/// ```rust
/// use yad_core::core::Value;
/// use yad_core::ffi::value::cstring_len_from_value;
///
/// let mut val = Value::from_string("hello".to_string()).unwrap();
/// let len = unsafe { cstring_len_from_value(&mut val as *mut Value) };
/// assert_eq!(len, 5);
/// ```
///
/// # FFI Notes
/// - This function uses `#[no_mangle]` and `extern "C"` to expose a stable symbol for C.
/// - The function does not allocate memory; it only reads the length of the string.
/// - Returns `0` for null pointers or non-string values to avoid undefined behavior.
#[unsafe(no_mangle)]
pub extern "C" fn cstring_len_from_value(value: *mut Value) -> usize {
    if value.is_null() {
        return 0;
    }
    unsafe {
        if let Ok(string) = (&*value).as_string() {
            string.len()
        } else {
            0
        }
    }
}

