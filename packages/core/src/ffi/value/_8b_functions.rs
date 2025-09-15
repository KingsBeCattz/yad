use float8::F8E4M3;
use crate::Value;

/// Converts an 8-bit unsigned integer (`u8`) into a heap-allocated [`Value`] pointer
/// suitable for FFI usage.
///
/// # Parameters
/// - `val`: The 8-bit unsigned integer to wrap.
///
/// # Returns
/// A raw pointer to a [`Value`] containing the provided `u8`.
///
/// # Safety
/// - The caller is responsible for freeing the returned pointer to avoid memory leaks.
/// - The pointer is valid for FFI usage but must not be dereferenced without validation.
#[unsafe(no_mangle)]
pub extern "C" fn value_from_uint_8(val: u8) -> *mut Value {
    Box::into_raw(Box::new(Value::from(val)))
}

/// Converts an 8-bit signed integer (`i8`) into a heap-allocated [`Value`] pointer
/// suitable for FFI usage.
///
/// # Parameters
/// - `val`: The 8-bit signed integer to wrap.
///
/// # Returns
/// A raw pointer to a [`Value`] containing the provided `i8`.
///
/// # Safety
/// - The caller must free the returned pointer to avoid memory leaks.
#[unsafe(no_mangle)]
pub extern "C" fn value_from_int_8(val: i8) -> *mut Value {
    Box::into_raw(Box::new(Value::from(val)))
}

/// Converts a 32-bit floating point number (`f32`) into a compact [`F8E4M3`] format,
/// then wraps it in a heap-allocated [`Value`] pointer for FFI.
///
/// # Parameters
/// - `val`: The `f32` value to convert.
///
/// # Returns
/// A raw pointer to a [`Value`] containing the `F8E4M3` representation.
///
/// # Safety
/// - The caller must free the pointer to avoid memory leaks.
/// - Precision may be lost due to the reduced bit representation of `F8E4M3`.
#[unsafe(no_mangle)]
pub extern "C" fn value_as_f8_from_float(val: f32) -> *mut Value {
    Box::into_raw(Box::new(Value::from(F8E4M3::from_f32(val))))
}

/// Attempts to extract a `u8` from a [`Value`] pointer and writes it to the provided output pointer.
///
/// # Parameters
/// - `value`: Pointer to the [`Value`] to extract from.
/// - `out`: Pointer to a `u8` where the result will be written.
///
/// # Returns
/// - `true` if extraction succeeded.
/// - `false` if either pointer is null or the conversion failed.
///
/// # Safety
/// - Both pointers must be valid and non-null.
/// - Dereferencing a null pointer is undefined behavior.
#[unsafe(no_mangle)]
pub extern "C" fn uint8_from_value(value: *mut Value, out: *mut u8) -> bool {
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

/// Attempts to extract an `i8` from a [`Value`] pointer and writes it to the provided output pointer.
///
/// # Parameters
/// - `value`: Pointer to the [`Value`] to extract from.
/// - `out`: Pointer to an `i8` where the result will be written.
///
/// # Returns
/// - `true` if extraction succeeded.
/// - `false` if either pointer is null or the conversion failed.
///
/// # Safety
/// - Both pointers must be valid and non-null.
/// - Dereferencing a null pointer is undefined behavior.
#[unsafe(no_mangle)]
pub extern "C" fn int8_from_value(value: *mut Value, out: *mut i8) -> bool {
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

/// Attempts to extract an [`F8E4M3`] floating point from a [`Value`] pointer,
/// convert it to `f32`, and write it to the provided output pointer.
///
/// # Parameters
/// - `value`: Pointer to the [`Value`] to extract from.
/// - `out`: Pointer to a `f32` where the result will be written.
///
/// # Returns
/// - `true` if extraction and conversion succeeded.
/// - `false` if either pointer is null or the conversion failed.
///
/// # Safety
/// - Both pointers must be valid and non-null.
/// - Dereferencing invalid pointers is undefined behavior.
/// - Precision may be lost due to the limited bits of the `F8E4M3` format.
#[unsafe(no_mangle)]
pub extern "C" fn float_from_f8_value(value: *mut Value, out: *mut f32) -> bool {
    if value.is_null() {
        return false;
    }
    unsafe {
        if let Ok(num) = <Value as TryInto<F8E4M3>>::try_into((&*value).to_owned()) {
            *out = num.to_f32();
            true
        } else {
            false
        }
    }
}
