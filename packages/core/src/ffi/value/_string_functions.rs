use std::ffi::{c_char, CStr, CString};
use crate::Value;

/// Creates a heap-allocated [`Value`] from a C string (`*const c_char`).
///
/// # Parameters
/// - `c_string`: Pointer to a null-terminated C string.
///
/// # Returns
/// - Pointer to a heap-allocated [`Value`] containing the string.
/// - Returns `null` if the input pointer is null or the conversion fails.
///
/// # Safety
/// - `c_string` must point to a valid null-terminated C string or be null.
/// - The returned pointer must eventually be freed with `value_free` to avoid memory leaks.
/// - Pointer must not be dereferenced after being freed.
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

    if let Ok(val) = Value::try_from(c_str) {
        Box::into_raw(Box::new(val))
    } else {
        std::ptr::null_mut()
    }
}

/// Converts a [`Value`] containing a Rust string into a C string (`*const c_char`).
///
/// # Parameters
/// - `value`: Pointer to a [`Value`] expected to contain a string.
///
/// # Returns
/// - Pointer to a null-terminated C string allocated on the heap.
/// - Returns `null` if `value` is null or conversion fails.
///
/// # Safety
/// - The returned C string must be freed using [`cstring_free`] when no longer needed.
/// - Pointer must not be used after being freed.
#[unsafe(no_mangle)]
pub extern "C" fn cstring_from_value(value: *mut Value) -> *const c_char {
    if value.is_null() {
        return std::ptr::null();
    }

    unsafe {
        match (&*value).clone().try_into() {
            Ok(string) => match CString::new::<String>(string) {
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
/// - `cstr`: Pointer to a heap-allocated C string.
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

/// Returns the length of a string contained within a [`Value`] as a C-compatible size.
///
/// # Parameters
/// - `value`: Pointer to a [`Value`] expected to contain a string. Can be null.
///
/// # Returns
/// - `usize`: Length of the string contained in the `Value`.
/// - Returns `0` if `value` is null or not a string.
///
/// # Safety
/// - `value` must be a valid pointer or null.
/// - The memory pointed to by `value` must remain valid for the duration of the call.
#[unsafe(no_mangle)]
pub extern "C" fn cstring_len_from_value(value: *mut Value) -> usize {
    if value.is_null() {
        return 0;
    }

    let string: Result<String, _> = unsafe { (&*value).clone().try_into() };
    string.map(|s| s.len()).unwrap_or(0)
}
