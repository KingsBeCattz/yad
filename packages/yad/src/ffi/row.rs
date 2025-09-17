use crate::{Row, Key};
use std::ffi::CStr;
use std::ptr;

/// # Row FFI (C ABI)
///
/// These functions provide a pure C-compatible interface for
/// creating, manipulating, and serializing [`Row`] objects.
///
/// All functions use `#[unsafe(no_mangle)]` to export symbols
/// compatible with a C toolchain. Memory allocated by these
/// functions must be freed using `row_free` to avoid leaks.

/// Creates a new [`Row`] from a C string and a vector of [`Key`] pointers.
///
/// # Safety
/// - `name` must be a valid null-terminated C string.
/// - `keys` is a pointer to an array of [`Key`] pointers of length `keys_len`.
/// - Any null pointer in `keys` array is ignored.
/// - Returns a null pointer on error.
///
/// # Parameters
/// - `name`: C string representing the row name.
/// - `keys`: Pointer to an array of [`Key`] pointers.
/// - `keys_len`: Length of the `keys` array.
///
/// # Returns
/// - Pointer to a heap-allocated [`Row`], must be freed with `row_free`.
#[unsafe(no_mangle)]
pub extern "C" fn row_new(name: *const i8, keys: *const *mut Key, keys_len: usize) -> *mut Row {
    unsafe {
        if name.is_null() { return ptr::null_mut(); }
        let cstr = match CStr::from_ptr(name).to_str() { Ok(s) => s, Err(_) => return ptr::null_mut() };

        let mut keys_vec = Vec::with_capacity(keys_len);
        if !keys.is_null() {
            for i in 0..keys_len {
                let key_ptr = *keys.add(i);
                if !key_ptr.is_null() {
                    keys_vec.push((*key_ptr).clone());
                }
            }
        }

        Box::into_raw(Box::new(Row::new(cstr, keys_vec)))
    }
}

/// Frees a [`Row`] previously allocated by `row_new`.
///
/// # Safety
/// - `row` must be a valid pointer returned by `row_new`.
/// - Passing a null pointer is safe and does nothing.
#[unsafe(no_mangle)]
pub extern "C" fn row_free(row: *mut Row) {
    unsafe { if !row.is_null() { let _ = Box::from_raw(row); } }
}

/// Inserts a [`Key`] into the [`Row`].
///
/// If a key with the same name exists, it will be replaced.
///
/// # Safety
/// - `row` must be a valid pointer to a [`Row`].
/// - `key` must be a valid pointer to a [`Key`].
#[unsafe(no_mangle)]
pub extern "C" fn row_insert_key(row: *mut Row, key: *mut Key) {
    unsafe {
        if row.is_null() || key.is_null() { return; }
        let row = &mut *row;
        let key = &*key;
        row.keys.insert(key.name.clone(), key.clone());
    }
}

/// Removes a [`Key`] from the [`Row`] by name.
///
/// # Safety
/// - `row` must be a valid pointer to a [`Row`].
/// - `name` must be a valid null-terminated C string.
///
/// # Returns
/// - Pointer to the removed [`Key`], or null if not found.
/// - Caller is responsible for freeing the returned key using `key_free`.
#[unsafe(no_mangle)]
pub extern "C" fn row_remove_key(row: *mut Row, name: *const i8) -> *mut Key {
    unsafe {
        if row.is_null() || name.is_null() { return ptr::null_mut(); }
        let cstr = match CStr::from_ptr(name).to_str() { Ok(s) => s, Err(_) => return ptr::null_mut() };
        match (*row).keys.remove(cstr) {
            Some(key) => Box::into_raw(Box::new(key)),
            None => ptr::null_mut(),
        }
    }
}

/// Serializes a [`Row`] into an external byte buffer.
///
/// Copies at most `max_len` bytes into `out_bytes`.
///
/// # Safety
/// - `row` must be a valid pointer to a [`Row`].
/// - `out_bytes` must point to a valid writable buffer of at least `max_len` bytes.
///
/// # Returns
/// - Number of bytes written to the buffer.
#[unsafe(no_mangle)]
pub extern "C" fn row_serialize(row: *const Row, out_bytes: *mut u8, max_len: usize) -> usize {
    unsafe {
        if row.is_null() || out_bytes.is_null() { return 0; }
        let row = &*row;
        match row.serialize() {
            Ok(vec) => {
                let len = vec.len().min(max_len);
                ptr::copy_nonoverlapping(vec.as_ptr(), out_bytes, len);
                len
            }
            Err(_) => 0,
        }
    }
}

/// Deserializes a [`Row`] from a byte buffer.
///
/// # Safety
/// - `bytes` must point to a valid buffer of length `len`.
/// - Returns null pointer if deserialization fails.
///
/// # Parameters
/// - `bytes`: Pointer to serialized row bytes.
/// - `len`: Number of bytes in the buffer.
///
/// # Returns
/// - Pointer to a newly allocated [`Row`], or null on error.
#[unsafe(no_mangle)]
pub extern "C" fn row_deserialize(bytes: *const u8, len: usize) -> *mut Row {
    unsafe {
        if bytes.is_null() || len == 0 { return ptr::null_mut(); }
        let vec = std::slice::from_raw_parts(bytes, len).to_vec();
        match Row::deserialize(vec) {
            Ok(row) => Box::into_raw(Box::new(row)),
            Err(_) => ptr::null_mut(),
        }
    }
}

/// Returns the number of keys in the [`Row`].
///
/// # Safety
/// - `row` must be a valid pointer to a [`Row`].
#[unsafe(no_mangle)]
pub extern "C" fn row_key_count(row: *const Row) -> usize {
    unsafe {
        if row.is_null() { return 0; }
        (*row).keys.len()
    }
}
