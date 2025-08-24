use crate::core::Value;

/// Creates a heap-allocated `Value` from a Rust boolean.
///
/// # Parameters
/// - `val`: Rust `bool` to convert into a `Value`.
///
/// # Returns
/// - Pointer to a heap-allocated `Value` representing the boolean.
/// - Returns `null` if conversion fails.
///
/// # Safety
/// - The returned pointer must eventually be freed with `value_free` to avoid memory leaks.
#[unsafe(no_mangle)]
pub extern "C" fn value_from_bool(val: bool) -> *mut Value {
    match Value::from_bool(val) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Extracts a Rust boolean from a `Value`.
///
/// # Parameters
/// - `value`: Pointer to a `Value` to read from.
/// - `out`: Pointer to a `bool` where the result will be written.
///
/// # Returns
/// - `true` if the conversion succeeded.
/// - `false` if the pointer is null or the `Value` is not a boolean.
///
/// # Safety
/// - `value` and `out` must be valid pointers.
/// - Writing to `out` assumes the caller provides valid memory.
#[unsafe(no_mangle)]
pub extern "C" fn bool_from_value(value: *mut Value, out: *mut bool) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(bool) = (&*value).as_bool() {
            *out = bool
        } else {
            return false;
        }
    }
    true
}
