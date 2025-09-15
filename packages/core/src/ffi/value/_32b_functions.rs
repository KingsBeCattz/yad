use crate::Value;

/// Creates a new [`Value`] containing an unsigned 32-bit integer (`u32`)
/// and returns a raw pointer suitable for FFI.
///
/// # Parameters
/// - `val`: The `u32` value to wrap.
///
/// # Returns
/// A raw pointer to a heap-allocated [`Value`] containing `val`.
/// Caller is responsible for freeing this pointer to prevent memory leaks.
///
/// # Safety
/// - The pointer is heap-allocated and must be manually deallocated.
/// - Must not be dereferenced after being freed.
#[unsafe(no_mangle)]
pub extern "C" fn value_from_uint_32(val: u32) -> *mut Value {
    Box::into_raw(Box::new(Value::from(val)))
}

/// Creates a new [`Value`] containing a signed 32-bit integer (`i32`)
/// and returns a raw pointer suitable for FFI.
///
/// # Parameters
/// - `val`: The `i32` value to wrap.
///
/// # Returns
/// A raw pointer to a heap-allocated [`Value`] containing `val`.
/// Caller is responsible for freeing this pointer.
///
/// # Safety
/// - The pointer must be manually deallocated to prevent memory leaks.
/// - Must not be used after being freed.
#[unsafe(no_mangle)]
pub extern "C" fn value_from_int_32(val: i32) -> *mut Value {
    Box::into_raw(Box::new(Value::from(val)))
}

/// Creates a new [`Value`] containing a 32-bit floating point (`f32`)
/// and returns a raw pointer suitable for FFI.
///
/// # Parameters
/// - `val`: The `f32` value to wrap.
///
/// # Returns
/// A raw pointer to a heap-allocated [`Value`] containing `val`.
/// Caller is responsible for freeing this pointer.
///
/// # Safety
/// - The pointer is heap-allocated and must be manually freed.
/// - Must not be dereferenced after being freed.
#[unsafe(no_mangle)]
pub extern "C" fn value_from_float(val: f32) -> *mut Value {
    Box::into_raw(Box::new(Value::from(val)))
}

/// Extracts a 32-bit floating point (`f32`) from a [`Value`] and writes it
/// into the provided pointer.
///
/// # Parameters
/// - `value`: Pointer to a [`Value`] expected to contain an `f32`.
/// - `out`: Pointer to a `f32` where the extracted value will be stored.
///
/// # Returns
/// - `true` if extraction succeeds.
/// - `false` if `value` is null or does not contain a valid `f32`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - `out` must point to a valid writable memory location.
#[unsafe(no_mangle)]
pub extern "C" fn float_from_value(value: *mut Value, out: *mut f32) -> bool {
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

/// Extracts an unsigned 32-bit integer (`u32`) from a [`Value`] and writes
/// it into the provided pointer.
///
/// # Parameters
/// - `value`: Pointer to a [`Value`] expected to contain a `u32`.
/// - `out`: Pointer to a `u32` where the extracted value will be stored.
///
/// # Returns
/// - `true` if extraction succeeds.
/// - `false` if `value` is null or does not contain a valid `u32`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - `out` must point to a valid writable memory location.
#[unsafe(no_mangle)]
pub extern "C" fn uint32_from_value(value: *mut Value, out: *mut u32) -> bool {
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

/// Extracts a signed 32-bit integer (`i32`) from a [`Value`] and writes
/// it into the provided pointer.
///
/// # Parameters
/// - `value`: Pointer to a [`Value`] expected to contain an `i32`.
/// - `out`: Pointer to an `i32` where the extracted value will be stored.
///
/// # Returns
/// - `true` if extraction succeeds.
/// - `false` if `value` is null or does not contain a valid `i32`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - `out` must point to a valid writable memory location.
#[unsafe(no_mangle)]
pub extern "C" fn int32_from_value(value: *mut Value, out: *mut i32) -> bool {
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
