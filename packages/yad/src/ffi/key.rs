use crate::Key;
use crate::Value;
use std::ffi::CStr;
use std::ptr;

/// # Key FFI (C ABI)
///
/// These functions provide a pure C-compatible interface for
/// creating, manipulating, and serializing [`Key`] objects.
///
/// All functions use `#[unsafe(no_mangle)]` to export symbols
/// compatible with a C toolchain. All memory allocations must
/// be freed using the corresponding `_free` functions to prevent
/// leaks.

/// Creates a new [`Key`] instance from a C string and a [`Value`].
///
/// # Safety
/// - `name` must be a valid null-terminated C string.
/// - `value` must be a valid pointer to a [`Value`] object.
/// - Returns a null pointer on error.
///
/// # Parameters
/// - `name`: C string pointer representing the key name.
/// - `value`: Pointer to a [`Value`] object.
///
/// # Returns
/// - Pointer to a heap-allocated [`Key`] object. Must be freed with `key_free`.
#[unsafe(no_mangle)]
pub extern "C" fn key_new(name: *const i8, value: *const Value) -> *mut Key {
    unsafe {
        if name.is_null() || value.is_null() { return ptr::null_mut(); }
        let cstr = CStr::from_ptr(name);
        let name_str = match cstr.to_str() { Ok(s) => s, Err(_) => return ptr::null_mut() };
        Box::into_raw(Box::new(Key::new(name_str, (*value).clone())))
    }
}

/// Frees a [`Key`] previously allocated by `key_new`.
///
/// # Safety
/// - `key` must be a valid pointer returned by `key_new`.
/// - Passing a null pointer is safe and has no effect.
#[unsafe(no_mangle)]
pub extern "C" fn key_free(key: *mut Key) {
    unsafe { if !key.is_null() { let _ = Box::from_raw(key); } }
}

/// Serializes a [`Key`] to an external byte buffer.
///
/// Copies at most `max_len` bytes into `out_bytes`.
///
/// # Safety
/// - `key` must be a valid pointer to a [`Key`].
/// - `out_bytes` must point to a valid writable buffer of at least `max_len` bytes.
///
/// # Returns
/// - Number of bytes written to the buffer.
#[unsafe(no_mangle)]
pub extern "C" fn key_serialize(key: *const Key, out_bytes: *mut u8, max_len: usize) -> usize {
    unsafe {
        if key.is_null() || out_bytes.is_null() { return 0; }
        let key = &*key;
        match key.serialize() {
            Ok(vec) => {
                let len = vec.len().min(max_len);
                ptr::copy_nonoverlapping(vec.as_ptr(), out_bytes, len);
                len
            }
            Err(_) => 0,
        }
    }
}

/// Deserializes a [`Key`] from a byte buffer.
///
/// # Safety
/// - `bytes` must point to a valid buffer of length `len`.
/// - Returns null pointer if deserialization fails.
///
/// # Parameters
/// - `bytes`: Pointer to serialized key bytes.
/// - `len`: Number of bytes in the buffer.
///
/// # Returns
/// - Pointer to a newly allocated [`Key`], or null on error.
#[unsafe(no_mangle)]
pub extern "C" fn key_deserialize(bytes: *const u8, len: usize) -> *mut Key {
    unsafe {
        if bytes.is_null() || len == 0 { return ptr::null_mut(); }
        let vec = std::slice::from_raw_parts(bytes, len).to_vec();
        match Key::deserialize(vec) {
            Ok(k) => Box::into_raw(Box::new(k)),
            Err(_) => ptr::null_mut(),
        }
    }
}

/// Returns a pointer to the name of the [`Key`] as a C string.
///
/// # Safety
/// - `key` must be a valid pointer to a [`Key`].
/// - Returned pointer is valid as long as the `Key` is alive.
/// - Do **not** free the returned pointer.
///
/// # Returns
/// - `const char*` pointer to the key's name.
#[unsafe(no_mangle)]
pub extern "C" fn key_get_name(key: *const Key) -> *const i8 {
    unsafe {
        if key.is_null() { return ptr::null(); }
        (*key).name.as_ptr() as *const i8
    }
}

/// Updates the [`Value`] of the given [`Key`].
///
/// # Safety
/// - Both `key` and `value` must be valid pointers.
/// - `key` must be mutable.
///
/// # Parameters
/// - `key`: Pointer to the [`Key`] to update.
/// - `value`: Pointer to the new [`Value`].
#[unsafe(no_mangle)]
pub extern "C" fn key_set_value(key: *mut Key, value: *const Value) {
    unsafe {
        if key.is_null() || value.is_null() { return; }
        (*key).set_value((*value).clone());
    }
}

/// Returns a pointer to the [`Value`] of the given [`Key`].
///
/// # Safety
/// - `key` must be a valid pointer to a [`Key`].
/// - Pointer is valid as long as `Key` is alive.
///
/// # Returns
/// - Pointer to the internal [`Value`].
#[unsafe(no_mangle)]
pub extern "C" fn key_get_value(key: *const Key) -> *const Value {
    unsafe {
        if key.is_null() { return ptr::null(); }
        &(*key).value
    }
}
