use float16::f16;
use crate::Value;

/// Creates a new [`Value`] containing an unsigned 16-bit integer (`u16`)
/// and returns a raw pointer suitable for FFI.
///
/// # Parameters
/// - `val`: The `u16` value to wrap.
///
/// # Returns
/// A raw pointer to a heap-allocated [`Value`] containing `val`.
/// The caller is responsible for freeing this pointer to prevent memory leaks.
///
/// # Safety
/// - The pointer is allocated on the heap and must be manually deallocated.
/// - Must not be dereferenced after being freed.
#[unsafe(no_mangle)]
pub extern "C" fn value_from_uint_16(val: u16) -> *mut Value {
    Box::into_raw(Box::new(Value::from(val)))
}

/// Creates a new [`Value`] containing a signed 16-bit integer (`i16`)
/// and returns a raw pointer suitable for FFI.
///
/// # Parameters
/// - `val`: The `i16` value to wrap.
///
/// # Returns
/// A raw pointer to a heap-allocated [`Value`] containing `val`.
/// The caller is responsible for freeing this pointer to prevent memory leaks.
///
/// # Safety
/// - The pointer must be properly deallocated.
/// - Must not be dereferenced after being freed.
#[unsafe(no_mangle)]
pub extern "C" fn value_from_int_16(val: i16) -> *mut Value {
    Box::into_raw(Box::new(Value::from(val)))
}

/// Creates a new [`Value`] representing a 16-bit floating point (`f16`)
/// by converting a 32-bit float (`f32`) to `f16`.
///
/// # Parameters
/// - `val`: The `f32` value to convert.
///
/// # Returns
/// A raw pointer to a heap-allocated [`Value`] containing the `f16`.
/// The caller is responsible for freeing this pointer.
///
/// # Safety
/// - The pointer must be manually deallocated.
/// - Must not be dereferenced after being freed.
/// - Precision loss may occur during the conversion from `f32` to `f16`.
#[unsafe(no_mangle)]
pub extern "C" fn value_as_f16_from_float(val: f32) -> *mut Value {
    Box::into_raw(Box::new(Value::from(f16::from_f32(val))))
}

/// Extracts a 16-bit floating point value (`f16`) from a [`Value`] and
/// writes it as a 32-bit float (`f32`) into the provided pointer.
///
/// # Parameters
/// - `value`: Pointer to a [`Value`] expected to contain an `f16`.
/// - `out`: Pointer to a `f32` where the result will be stored.
///
/// # Returns
/// - `true` if extraction and conversion succeed.
/// - `false` if `value` is null or does not contain a valid `f16`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - `out` must point to a valid writable memory location.
#[unsafe(no_mangle)]
pub extern "C" fn float_from_f16_value(value: *mut Value, out: *mut f32) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(num) = <Value as TryInto<f16>>::try_into((&*value).to_owned()) {
            *out = num.to_f32();
            true
        } else {
            false
        }
    }
}

/// Extracts an unsigned 16-bit integer (`u16`) from a [`Value`] and writes
/// it into the provided pointer.
///
/// # Parameters
/// - `value`: Pointer to a [`Value`] expected to contain a `u16`.
/// - `out`: Pointer to a `u16` where the result will be stored.
///
/// # Returns
/// - `true` if extraction succeeds.
/// - `false` if `value` is null or does not contain a valid `u16`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - `out` must point to a valid writable memory location.
#[unsafe(no_mangle)]
pub extern "C" fn uint16_from_value(value: *mut Value, out: *mut u16) -> bool {
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

/// Extracts a signed 16-bit integer (`i16`) from a [`Value`] and writes
/// it into the provided pointer.
///
/// # Parameters
/// - `value`: Pointer to a [`Value`] expected to contain an `i16`.
/// - `out`: Pointer to an `i16` where the result will be stored.
///
/// # Returns
/// - `true` if extraction succeeds.
/// - `false` if `value` is null or does not contain a valid `i16`.
///
/// # Safety
/// - Both `value` and `out` must be valid, non-null pointers.
/// - `out` must point to a valid writable memory location.
#[unsafe(no_mangle)]
pub extern "C" fn int16_from_value(value: *mut Value, out: *mut i16) -> bool {
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
