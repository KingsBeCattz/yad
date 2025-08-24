use std::collections::HashMap;
use std::ffi::{c_char, CStr};
use crate::core::{Key, Row};

/// Frees a heap-allocated `Row`.
///
/// # Parameters
/// - `val`: Pointer to the `Row` to free.
///
/// # Safety
/// - `val` must be a pointer previously returned from FFI or null.
/// - After calling this function, `val` must not be used again.
#[unsafe(no_mangle)]
pub extern "C" fn row_free(val: *mut Row) {
    if !val.is_null() {
        unsafe { let _ = Box::from_raw(val); }
    }
}

/// Creates a new `Row` with a given name.
///
/// # Parameters
/// - `c_name`: Pointer to a null-terminated C string containing the row name.
///
/// # Returns
/// - Pointer to a heap-allocated `Row`.
/// - Returns `null` if `c_name` is null or invalid.
///
/// # Safety
/// - The returned pointer must eventually be freed with `row_free` to avoid memory leaks.
#[unsafe(no_mangle)]
pub extern "C" fn row_new(c_name: *const c_char) -> *mut Row {
    if c_name.is_null() {
        return std::ptr::null_mut()
    }

    let name = unsafe { CStr::from_ptr(c_name) };
    let name = match name.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    Box::into_raw(Box::new(Row::new(name.to_string(), HashMap::new())))
}

/// Returns the number of keys in a `Row`.
///
/// # Parameters
/// - `row`: Pointer to a `Row`.
///
/// # Returns
/// - Number of keys in the row, or 0 if `row` is null.
#[unsafe(no_mangle)]
pub extern "C" fn row_keys_len(row: *mut Row) -> usize {
    if row.is_null() {
        return 0
    }

    unsafe {
        (&*row).keys.len()
    }
}

/// Inserts a `Key` into a `Row`.
///
/// # Parameters
/// - `row`: Pointer to a `Row`.
/// - `to_insert`: Pointer to a heap-allocated `Key` to insert.
///
/// # Returns
/// - `true` if insertion succeeded.
/// - `false` if either pointer is null.
///
/// # Safety
/// - Ownership of `to_insert` is transferred to the row.
#[unsafe(no_mangle)]
pub extern "C" fn row_insert_key(row: *mut Row, to_insert: *mut Key) -> bool {
    if row.is_null() || to_insert.is_null() {
        return false;
    }

    unsafe {
        let row_ref = &mut *row;
        let map = &mut row_ref.keys;
        let insert_this: Box<Key> = Box::from_raw(to_insert);

        map.insert(insert_this.name.clone(), *insert_this);

        true
    }
}

/// Removes a `Key` from a `Row` by name.
///
/// # Parameters
/// - `row`: Pointer to a `Row`.
/// - `name`: Pointer to a null-terminated C string containing the key name.
///
/// # Returns
/// - `true` if removal succeeded.
/// - `false` if either pointer is null or the name cannot be converted.
#[unsafe(no_mangle)]
pub extern "C" fn row_remove_key(row: *mut Row, name: *const c_char) -> bool {
    if row.is_null() || name.is_null() {
        return false;
    }

    unsafe {
        let row_ref = &mut *row;
        let map = &mut row_ref.keys;

        let c_str = match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };

        map.remove(c_str);

        true
    }
}

/// Retrieves a mutable pointer to a `Key` in a `Row` by name.
///
/// # Parameters
/// - `row`: Pointer to a `Row`.
/// - `name`: Pointer to a null-terminated C string containing the key name.
///
/// # Returns
/// - Pointer to the `Key` if found, or `null` otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn row_get_key(row: *mut Row, name: *const c_char) -> *mut Key {
    if row.is_null() || name.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let row_ref = &mut *row;
        let map = &mut row_ref.keys;

        let c_str = match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };

        if let Some(row) = map.get_mut(c_str) {
            row as *mut Key
        } else {
            std::ptr::null_mut()
        }
    }
}

/// Serializes a `Row` into a raw byte buffer.
///
/// # Parameters
/// - `row`: Pointer to a `Row`.
///
/// # Returns
/// - Pointer to a buffer containing the serialized row, or null on error.
/// - The buffer is heap-allocated and must remain valid until manually freed.
#[unsafe(no_mangle)]
pub extern "C" fn row_as_buffer(row: *mut Row) -> *const u8 {
    if row.is_null() {
        return std::ptr::null();
    }

    unsafe {
        match (&*row).encode() {
            Ok(buff) => {
                let boxed = buff.into_boxed_slice();
                let ptr = boxed.as_ptr();
                std::mem::forget(boxed);
                ptr
            }
            Err(_) => std::ptr::null(),
        }
    }
}

/// Deserializes a `Row` from a raw byte buffer.
///
/// # Parameters
/// - `buff`: Pointer to a buffer containing serialized row data.
/// - `len`: Length of the buffer.
///
/// # Returns
/// - Pointer to a heap-allocated `Row`, or null if deserialization fails.
#[unsafe(no_mangle)]
pub extern "C" fn row_from_buffer(buff: *const u8, len: usize) -> *mut Row {
    if buff.is_null() || len == 0 {
        return std::ptr::null_mut();
    }

    unsafe {
        let slice = std::slice::from_raw_parts(buff, len);
        let vec = slice.to_vec();

        match Row::decode(vec) {
            Ok(buff) => Box::into_raw(Box::new(buff)),
            Err(_) => std::ptr::null_mut(),
        }
    }
}
