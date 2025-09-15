use crate::Value;

/// Creates a new [`Value`] containing an unsigned 64-bit integer (`u64`)
/// and returns a raw pointer suitable for FFI.
///
/// # Parameters
/// - `val`: The `u64` value to wrap.
///
/// # Returns
/// A raw pointer to a heap-allocated [`Value`] containing `val`.
/// Caller is responsible for freeing this pointer to avoid memory leaks.
///
/// # Safety
/// - The pointer is heap-allocated and must be manually deallocated.
/// - Must not be dereferenced after being freed.
#[unsafe(no_mangle)]
pub extern "C" fn value_from_uint_64(val: u64) -> *mut Value {
    Box::into_raw(Box::new(Value::from(val)))
}

/// Creates a new [`Value`] containing a signed 64-bit integer (`i64`)
/// and returns a raw pointer suitable for FFI.
///
/// # Parameters
/// - `val`: The `i64` value to wrap.
///
/// # Returns
/// A raw pointer to a heap-allocated [`Value`] containing `val`.
/// Caller is responsible for freeing this pointer.
///
/// # Safety
/// - The pointer must be manually managed by the caller.
/// - Must not be dereferenced after being freed.
#[unsafe(no_mangle)]
pub extern "C" fn value_from_int_64(val: i64) -> *mut Value {
    Box::into_raw(Box::new(Value::from(val)))
}

/// Creates a new [`Value`] containing a 64-bit floating point (`f64`)
/// and returns a raw pointer suitable for FFI.
///
/// # Parameters
/// - `val`: The `f64` value to wrap.
///
/// # Returns
/// A raw pointer to a heap-allocated [`Value`] containing `val`.
/// Caller is responsible for freeing this pointer.
///
/// # Safety
/// - The pointer is heap-allocated and must be manually freed.
/// - Must not be dereferenced after being freed.
#[unsafe(no_mangle)]
pub extern "C" fn value_from_double(val: f64) -> *mut Value {
    Box::into_raw(Box::new(Value::from(val)))
}

/// Extracts a 64-bit floating point (`f64`) from a [`Value`] and writes it
/// into the provided pointer.
///
/// # Parameters
/// - `value`: Pointer to a [`Value`] expected to contain an `f64`.
/// - `out`: Pointer to a `f64` where the extracted value will be stored.
///
/// # Returns
/// - `true` if extraction succeeds.
/// - `false` if `value` is null or does not contain a valid `f64`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - `out` must point to a valid writable memory location.
#[unsafe(no_mangle)]
pub extern "C" fn double_from_value(value: *mut Value, out: *mut f64) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(num) = (&*value).try_into() {
            *out = num;
            true
        } else {
            false
        }
    }
}

/// Extracts an unsigned 64-bit integer (`u64`) from a [`Value`] and writes
/// it into the provided pointer.
///
/// # Parameters
/// - `value`: Pointer to a [`Value`] expected to contain a `u64`.
/// - `out`: Pointer to a `u64` where the extracted value will be stored.
///
/// # Returns
/// - `true` if extraction succeeds.
/// - `false` if `value` is null or does not contain a valid `u64`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - `out` must point to a valid writable memory location.
#[unsafe(no_mangle)]
pub extern "C" fn uint64_from_value(value: *mut Value, out: *mut u64) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(num) = (&*value).try_into() {
            *out = num;
            true
        } else {
            false
        }
    }
}

/// Extracts a signed 64-bit integer (`i64`) from a [`Value`] and writes
/// it into the provided pointer.
///
/// # Parameters
/// - `value`: Pointer to a [`Value`] expected to contain an `i64`.
/// - `out`: Pointer to an `i64` where the extracted value will be stored.
///
/// # Returns
/// - `true` if extraction succeeds.
/// - `false` if `value` is null or does not contain a valid `i64`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - `out` must point to a valid writable memory location.
#[unsafe(no_mangle)]
pub extern "C" fn int64_from_value(value: *mut Value, out: *mut i64) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(num) = (&*value).try_into() {
            *out = num;
            true
        } else {
            false
        }
    }
}
