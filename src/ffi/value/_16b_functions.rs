use float16::f16;
use crate::core::Value;

#[unsafe(no_mangle)]
/// Creates a new [`Value`] instance from an unsigned 16-bit integer (`u16`)
/// and returns a raw pointer to it for FFI interoperability.
///
/// # Parameters
/// - `val`: The unsigned 16-bit integer (`u16`) to wrap inside a [`Value`].
///
/// # Returns
/// - `*mut Value`: A raw pointer to the allocated [`Value`] containing `val`.
///   The caller is responsible for freeing this memory using the appropriate
///   deallocation function, otherwise a memory leak may occur.
///
/// # Safety
/// - The returned pointer is allocated on the heap and must be managed manually.
/// - Must not be dereferenced after being freed.
pub extern "C" fn value_from_uint_16(val: u16) -> *mut Value {
    Box::into_raw(Box::new(Value::from_u16(val)))
}

#[unsafe(no_mangle)]
/// Creates a new [`Value`] instance from a signed 16-bit integer (`i16`)
/// and returns a raw pointer to it for FFI interoperability.
///
/// # Parameters
/// - `val`: The signed 16-bit integer (`i16`) to wrap inside a [`Value`].
///
/// # Returns
/// - `*mut Value`: A raw pointer to the allocated [`Value`] containing `val`.
///   The caller is responsible for freeing this memory.
///
/// # Safety
/// - The returned pointer must be properly deallocated to prevent memory leaks.
/// - Caller must ensure pointer validity when used across FFI boundaries.
pub extern "C" fn value_from_int_16(val: i16) -> *mut Value {
    Box::into_raw(Box::new(Value::from_i16(val)))
}

#[unsafe(no_mangle)]
/// Creates a new [`Value`] instance representing a 16-bit floating point (`f16`)
/// by converting a 32-bit floating point (`f32`) into `f16`.
///
/// # Parameters
/// - `val`: The 32-bit float (`f32`) to be converted and stored as `f16`.
///
/// # Returns
/// - `*mut Value`: A raw pointer to the allocated [`Value`] containing the `f16`.
///   Caller is responsible for freeing this pointer.
///
/// # Safety
/// - Returned pointer must not be dereferenced after deallocation.
/// - Memory management is required on the caller side.
pub extern "C" fn value_as_f16_from_float(val: f32) -> *mut Value {
    Box::into_raw(Box::new(Value::from_f16(f16::from_f32(val))))
}

#[unsafe(no_mangle)]
/// Extracts a 16-bit floating point value (`f16`) from a [`Value`] and writes
/// it as a 32-bit float (`f32`) into the provided output pointer.
///
/// # Parameters
/// - `value`: A raw pointer to a [`Value`] expected to contain an `f16`.
/// - `out`: A mutable pointer where the resulting `f32` will be stored.
///
/// # Returns
/// - `true` if the extraction and conversion succeed.
/// - `false` if `value` is `null` or does not contain a valid `f16`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - Caller must ensure that `out` points to a valid writable memory location.
pub extern "C" fn float_from_f16_value(value: *mut Value, out: *mut f32) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(num) = (&*value).as_f16() {
            *out = num.to_f32()
        } else {
            return false;
        }
    }
    true
}

#[unsafe(no_mangle)]
/// Extracts an unsigned 16-bit integer (`u16`) from a [`Value`] and writes
/// it into the provided output pointer.
///
/// # Parameters
/// - `value`: A raw pointer to a [`Value`] expected to contain a `u16`.
/// - `out`: A mutable pointer where the extracted `u16` will be written.
///
/// # Returns
/// - `true` if extraction succeeds.
/// - `false` if `value` is `null` or does not contain a valid `u16`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - Caller must ensure that `out` points to a valid writable memory location.
pub extern "C" fn uint16_from_value(value: *mut Value, out: *mut u16) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(num) = (&*value).as_u16() {
            *out = num
        } else {
            return false;
        }
    }
    true
}

#[unsafe(no_mangle)]
/// Extracts a signed 16-bit integer (`i16`) from a [`Value`] and writes
/// it into the provided output pointer.
///
/// # Parameters
/// - `value`: A raw pointer to a [`Value`] expected to contain an `i16`.
/// - `out`: A mutable pointer where the extracted `i16` will be written.
///
/// # Returns
/// - `true` if extraction succeeds.
/// - `false` if `value` is `null` or does not contain a valid `i16`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - Caller must ensure that `out` points to a valid writable memory location.
pub extern "C" fn int16_from_value(value: *mut Value, out: *mut i16) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(num) = (&*value).as_i16() {
            *out = num
        } else {
            return false;
        }
    }
    true
}