use crate::Value;

/// Creates a heap-allocated [`Value`] from a Rust boolean.
///
/// # Parameters
/// - `val`: Rust `bool` to wrap inside a [`Value`].
///
/// # Returns
/// - Pointer to a heap-allocated [`Value`] representing the boolean.
/// - Never returns null in normal operation (conversion cannot fail for bools).
///
/// # Safety
/// - The returned pointer must eventually be freed with `value_free` to avoid memory leaks.
/// - Pointer must not be dereferenced after being freed.
#[unsafe(no_mangle)]
pub extern "C" fn value_from_bool(val: bool) -> *mut Value {
    Box::into_raw(Box::new(Value::from(val)))
}

/// Extracts a Rust boolean from a heap-allocated [`Value`].
///
/// # Parameters
/// - `value`: Pointer to a [`Value`] expected to contain a boolean.
/// - `out`: Pointer to a `bool` where the result will be written.
///
/// # Returns
/// - `true` if extraction succeeded.
/// - `false` if `value` is null or does not contain a valid boolean.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - Caller must ensure that `out` points to a valid writable memory location.
#[unsafe(no_mangle)]
pub extern "C" fn bool_from_value(value: *mut Value, out: *mut bool) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(b) = (&*value).clone().try_into() {
            *out = b;
            true
        } else {
            false
        }
    }
}
