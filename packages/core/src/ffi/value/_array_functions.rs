use crate::Value;
use crate::ffi::CArray;

/// Converts a C-compatible array (`CArray`) into a heap-allocated [`Value`].
///
/// # Parameters
/// - `c_arr`: Pointer to a `CArray` containing array elements.
///
/// # Returns
/// - A raw pointer to a heap-allocated [`Value`] containing the array data.
/// - Returns `null` if the input pointer is null or the conversion fails.
///
/// # Safety
/// - `c_arr` must be a valid pointer or null.
/// - The caller must free the returned pointer with `value_free` to avoid memory leaks.
/// - Ownership of the array memory is transferred temporarily; the original `CArray` should not be used after this call.
#[unsafe(no_mangle)]
pub extern "C" fn value_from_c_array(c_arr: *mut CArray) -> *mut Value {
    if c_arr.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        // Reconstruct Vec<Value> from raw parts
        let vec = Vec::from_raw_parts((*c_arr).ptr, (*c_arr).len, (*c_arr).cap);

        // Convert Vec<Value> into Value
        match Value::try_from(vec) {
            Ok(v) => Box::into_raw(Box::new(v)),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

/// Converts a heap-allocated [`Value`] containing an array into a C-compatible `CArray`.
///
/// # Parameters
/// - `val`: Pointer to a [`Value`] expected to contain an array.
///
/// # Returns
/// - A raw pointer to a heap-allocated `CArray` containing the array data.
/// - Returns `null` if the input pointer is null or the [`Value`] is not an array.
///
/// # Safety
/// - `val` must be a valid pointer or null.
/// - The returned `CArray` must be freed with `free_c_array` to avoid memory leaks.
/// - Memory inside the original [`Value`] remains managed by Rust; this exposes the array contents as a `CArray`.
#[unsafe(no_mangle)]
pub extern "C" fn c_array_from_value(val: *mut Value) -> *mut CArray {
    if val.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        match <Value as TryInto<Vec<Value>>>::try_into((&*val).clone()) {
            Ok(mut arr) => Box::into_raw(Box::new(CArray {
                ptr: arr.as_mut_ptr(),
                len: arr.len(),
                cap: arr.capacity(),
            })),
            Err(_) => std::ptr::null_mut(),
        }
    }
}
