use crate::core::Value;

#[unsafe(no_mangle)]
/// Creates a new [`Value`] instance from an unsigned 64-bit integer (`u64`)
/// and returns a raw pointer to it for FFI interoperability.
///
/// # Parameters
/// - `val`: The unsigned 64-bit integer (`u64`) to wrap inside a [`Value`].
///
/// # Returns
/// - `*mut Value`: A raw pointer to the allocated [`Value`] containing `val`.
///   Caller is responsible for freeing this pointer to avoid memory leaks.
///
/// # Safety
/// - The returned pointer is heap-allocated.
/// - Must be manually deallocated by the caller.
/// - Pointer must not be dereferenced after being freed.
pub extern "C" fn value_from_uint_64(val: u64) -> *mut Value {
    Box::into_raw(Box::new(Value::from_u64(val)))
}

#[unsafe(no_mangle)]
/// Creates a new [`Value`] instance from a signed 64-bit integer (`i64`)
/// and returns a raw pointer to it for FFI interoperability.
///
/// # Parameters
/// - `val`: The signed 64-bit integer (`i64`) to wrap inside a [`Value`].
///
/// # Returns
/// - `*mut Value`: A raw pointer to the allocated [`Value`] containing `val`.
///   Caller must deallocate this pointer to prevent memory leaks.
///
/// # Safety
/// - The returned pointer must be manually managed by the caller.
/// - Pointer must not be dereferenced after being freed.
pub extern "C" fn value_from_int_64(val: i64) -> *mut Value {
    Box::into_raw(Box::new(Value::from_i64(val)))
}

#[unsafe(no_mangle)]
/// Creates a new [`Value`] instance from a 64-bit floating point (`f64`)
/// and returns a raw pointer to it for FFI interoperability.
///
/// # Parameters
/// - `val`: The 64-bit float (`f64`) to wrap inside a [`Value`].
///
/// # Returns
/// - `*mut Value`: A raw pointer to the allocated [`Value`] containing `val`.
///   Caller is responsible for freeing this pointer.
///
/// # Safety
/// - The returned pointer is allocated on the heap.
/// - Caller must free this pointer to avoid leaks.
pub extern "C" fn value_from_double(val: f64) -> *mut Value {
    Box::into_raw(Box::new(Value::from_f64(val)))
}

#[unsafe(no_mangle)]
/// Extracts a 64-bit floating point (`f64`) from a [`Value`] and writes it
/// into the provided output pointer.
///
/// # Parameters
/// - `value`: A raw pointer to a [`Value`] expected to contain an `f64`.
/// - `out`: A mutable pointer where the extracted `f64` will be written.
///
/// # Returns
/// - `true` if extraction succeeds.
/// - `false` if `value` is `null` or does not contain a valid `f64`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - Caller must ensure that `out` points to a valid writable memory location.
pub extern "C" fn double_from_value(value: *mut Value, out: *mut f64) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(num) = (&*value).as_f64() {
            *out = num
        } else {
            return false;
        }
    }
    true
}

#[unsafe(no_mangle)]
/// Extracts an unsigned 64-bit integer (`u64`) from a [`Value`] and writes
/// it into the provided output pointer.
///
/// # Parameters
/// - `value`: A raw pointer to a [`Value`] expected to contain a `u64`.
/// - `out`: A mutable pointer where the extracted `u64` will be written.
///
/// # Returns
/// - `true` if extraction succeeds.
/// - `false` if `value` is `null` or does not contain a valid `u64`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - Caller must ensure that `out` points to writable memory.
pub extern "C" fn uint64_from_value(value: *mut Value, out: *mut u64) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(num) = (&*value).as_u64() {
            *out = num
        } else {
            return false;
        }
    }
    true
}

#[unsafe(no_mangle)]
/// Extracts a signed 64-bit integer (`i64`) from a [`Value`] and writes
/// it into the provided output pointer.
///
/// # Parameters
/// - `value`: A raw pointer to a [`Value`] expected to contain an `i64`.
/// - `out`: A mutable pointer where the extracted `i64` will be written.
///
/// # Returns
/// - `true` if extraction succeeds.
/// - `false` if `value` is `null` or does not contain a valid `i64`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - Caller must ensure that `out` points to writable memory.
pub extern "C" fn int64_from_value(value: *mut Value, out: *mut i64) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(num) = (&*value).as_i64() {
            *out = num
        } else {
            return false;
        }
    }
    true
}
