use crate::core::Value;

#[unsafe(no_mangle)]
/// Creates a new [`Value`] instance from an unsigned 32-bit integer (`u32`)
/// and returns a raw pointer to it for FFI interoperability.
///
/// # Parameters
/// - `val`: The unsigned 32-bit integer (`u32`) to wrap inside a [`Value`].
///
/// # Returns
/// - `*mut Value`: A raw pointer to the allocated [`Value`] containing `val`.
///   Caller is responsible for freeing this pointer to avoid memory leaks.
///
/// # Safety
/// - The returned pointer is allocated on the heap.
/// - Must be manually deallocated by the caller after use.
/// - Pointer must not be dereferenced after being freed.
pub extern "C" fn value_from_uint_32(val: u32) -> *mut Value {
    Box::into_raw(Box::new(Value::from_u32(val)))
}

#[unsafe(no_mangle)]
/// Creates a new [`Value`] instance from a signed 32-bit integer (`i32`)
/// and returns a raw pointer to it for FFI interoperability.
///
/// # Parameters
/// - `val`: The signed 32-bit integer (`i32`) to wrap inside a [`Value`].
///
/// # Returns
/// - `*mut Value`: A raw pointer to the allocated [`Value`] containing `val`.
///   Caller is responsible for freeing this pointer.
///
/// # Safety
/// - Pointer must be deallocated by the caller to prevent memory leaks.
/// - Must not be used after being freed.
pub extern "C" fn value_from_int_32(val: i32) -> *mut Value {
    Box::into_raw(Box::new(Value::from_i32(val)))
}

#[unsafe(no_mangle)]
/// Creates a new [`Value`] instance from a 32-bit floating point (`f32`)
/// and returns a raw pointer to it for FFI interoperability.
///
/// # Parameters
/// - `val`: The 32-bit float (`f32`) to wrap inside a [`Value`].
///
/// # Returns
/// - `*mut Value`: A raw pointer to the allocated [`Value`] containing `val`.
///   Caller is responsible for freeing this pointer.
///
/// # Safety
/// - The returned pointer is heap-allocated and must be freed manually.
/// - Do not dereference after freeing.
pub extern "C" fn value_from_float(val: f32) -> *mut Value {
    Box::into_raw(Box::new(Value::from_f32(val)))
}

#[unsafe(no_mangle)]
/// Extracts a 32-bit floating point (`f32`) from a [`Value`] and writes it
/// into the provided output pointer.
///
/// # Parameters
/// - `value`: A raw pointer to a [`Value`] expected to contain an `f32`.
/// - `out`: A mutable pointer where the extracted `f32` will be written.
///
/// # Returns
/// - `true` if extraction succeeds.
/// - `false` if `value` is `null` or does not contain a valid `f32`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - Caller must ensure that `out` points to a valid writable memory location.
pub extern "C" fn float_from_value(value: *mut Value, out: *mut f32) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(num) = (&*value).as_f32() {
            *out = num
        } else {
            return false;
        }
    }
    true
}

#[unsafe(no_mangle)]
/// Extracts an unsigned 32-bit integer (`u32`) from a [`Value`] and writes
/// it into the provided output pointer.
///
/// # Parameters
/// - `value`: A raw pointer to a [`Value`] expected to contain a `u32`.
/// - `out`: A mutable pointer where the extracted `u32` will be written.
///
/// # Returns
/// - `true` if extraction succeeds.
/// - `false` if `value` is `null` or does not contain a valid `u32`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - Caller must ensure that `out` points to writable memory.
pub extern "C" fn uint32_from_value(value: *mut Value, out: *mut u32) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(num) = (&*value).as_u32() {
            *out = num
        } else {
            return false;
        }
    }
    true
}

#[unsafe(no_mangle)]
/// Extracts a signed 32-bit integer (`i32`) from a [`Value`] and writes
/// it into the provided output pointer.
///
/// # Parameters
/// - `value`: A raw pointer to a [`Value`] expected to contain an `i32`.
/// - `out`: A mutable pointer where the extracted `i32` will be written.
///
/// # Returns
/// - `true` if extraction succeeds.
/// - `false` if `value` is `null` or does not contain a valid `i32`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - Caller must ensure that `out` points to writable memory.
pub extern "C" fn int32_from_value(value: *mut Value, out: *mut i32) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(num) = (&*value).as_i32() {
            *out = num
        } else {
            return false;
        }
    }
    true
}
