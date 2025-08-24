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
/// - `out`: Pointer to a `*const c_char` where the resulting C string pointer will be written.
///
/// # Returns
/// - `true` if the conversion succeeded.
/// - `false` if the `value` pointer is null, not a string, or conversion fails.
///
/// # Safety
/// - `value` and `out` must be valid pointers.
/// - The returned C string is allocated on the heap and must be freed using `CString::from_raw`
///   when no longer needed to avoid memory leaks.
#[unsafe(no_mangle)]
pub extern "C" fn cstring_from_value(value: *mut Value, out: *mut *const c_char) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(string) = (&*value).as_string() {
            if let Ok(c_str) = CString::new(string) {
                *out = c_str.into_raw()
            } else {
                return false;
            };
        } else {
            return false;
        }
    }
    true
}
