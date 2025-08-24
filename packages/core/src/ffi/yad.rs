use std::ffi::{c_char, CStr, CString};
use crate::core::{Row, YAD};
use crate::{deserialize, serialize};

/// Creates a new heap-allocated `YAD`.
///
/// # Returns
/// - Pointer to a heap-allocated `YAD`.
///
/// # Safety
/// - The returned pointer must eventually be freed with `yad_free` to avoid memory leaks.
#[unsafe(no_mangle)]
pub extern "C" fn yad_new() -> *mut YAD {
    Box::into_raw(Box::new(YAD::new()))
}

/// Frees a heap-allocated `YAD`.
///
/// # Parameters
/// - `yad`: Pointer to the `YAD` to free.
///
/// # Safety
/// - `yad` must be a pointer previously returned from FFI or null.
/// - After calling this function, `yad` must not be used again.
#[unsafe(no_mangle)]
pub extern "C" fn yad_free(yad: *mut YAD) {
    if !yad.is_null() {
        unsafe { let _ = Box::from_raw(yad); }
    }
}

/// Returns a pointer to the version bytes of a `YAD`.
///
/// # Parameters
/// - `yad`: Pointer to the `YAD`.
///
/// # Returns
/// - Pointer to the serialized version bytes.
/// - Returns a pointer to a zeroed array if `yad` is null.
///
/// # Safety
/// - The returned pointer is valid as long as the `YAD` exists.
/// - Modifying the bytes may lead to undefined behavior.
#[unsafe(no_mangle)]
pub extern "C" fn yad_version(yad: *mut YAD) -> *const u8 {
    if yad.is_null() {
        return [0,0,0,1].as_ptr()
    }

    unsafe {
        (&*yad).version.serialize()[1..].as_ptr()
    }
}

/// Returns the number of rows in a `YAD`.
///
/// # Parameters
/// - `yad`: Pointer to a `YAD`.
///
/// # Returns
/// - Number of rows, or 0 if `yad` is null.
#[unsafe(no_mangle)]
pub extern "C" fn yad_rows_len(yad: *mut YAD) -> usize {
    if yad.is_null() {
        return 0
    }

    unsafe {
        (&*yad).rows.len()
    }
}

/// Inserts a `Row` into a `YAD`.
///
/// # Parameters
/// - `yad`: Pointer to a `YAD`.
/// - `to_insert`: Pointer to a heap-allocated `Row` to insert.
///
/// # Returns
/// - `true` if insertion succeeded.
/// - `false` if either pointer is null.
///
/// # Safety
/// - Ownership of `to_insert` is transferred to the `YAD`.
#[unsafe(no_mangle)]
pub extern "C" fn yad_insert_row(yad: *mut YAD, to_insert: *mut Row) -> bool {
    if yad.is_null() || to_insert.is_null() {
        return false;
    }

    unsafe {
        let yad_ref = &mut *yad;
        let map = &mut yad_ref.rows;
        let insert_this: Box<Row> = Box::from_raw(to_insert);

        map.insert(insert_this.name.clone(), *insert_this);

        true
    }
}

/// Removes a `Row` from a `YAD` by name.
///
/// # Parameters
/// - `yad`: Pointer to a `YAD`.
/// - `name`: Pointer to a null-terminated C string containing the row name.
///
/// # Returns
/// - `true` if removal succeeded.
/// - `false` if either pointer is null or conversion fails.
#[unsafe(no_mangle)]
pub extern "C" fn yad_remove_row(yad: *mut YAD, name: *const c_char) -> bool {
    if yad.is_null() || name.is_null() {
        return false;
    }

    unsafe {
        let yad_ref = &mut *yad;
        let map = &mut yad_ref.rows;

        let c_str = match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };

        map.remove(c_str);

        true
    }
}

/// Retrieves a mutable pointer to a `Row` in a `YAD` by name.
///
/// # Parameters
/// - `yad`: Pointer to a `YAD`.
/// - `name`: Pointer to a null-terminated C string containing the row name.
///
/// # Returns
/// - Pointer to the `Row` if found, or `null` otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn yad_get_row(yad: *mut YAD, name: *const c_char) -> *mut Row {
    if yad.is_null() || name.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let yad_ref = &mut *yad;
        let map = &mut yad_ref.rows;

        let c_str = match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };

        if let Some(row) = map.get_mut(c_str) {
            row as *mut Row
        } else {
            std::ptr::null_mut()
        }
    }
}

/// Returns an array of C-compatible string pointers representing the row names of a YAD.
///
/// # Safety
/// - The caller must ensure that `yad` is a valid, non-null pointer to a `YAD`.
/// - The returned array and the strings it contains are heap-allocated and must be freed
///   using `yad_rows_names_free` to avoid memory leaks.
///
/// # Returns
/// - A pointer to a heap-allocated array of `*const c_char`.
/// - If `yad` is null, returns a null pointer.
///
/// # Notes
/// - Each string in the array is individually heap-allocated via `CString::into_raw`.
/// - The length of the array must be obtained via a separate function (`yad_rows_len`).
/// - Ownership of the array and strings is transferred to the caller; Rust will not
///   automatically free this memory.
#[unsafe(no_mangle)]
pub extern "C" fn yad_rows_names(yad: *mut YAD) -> *mut *const c_char {
    if yad.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let yad_ref = &*yad;

        // Preallocate a Vec to hold the raw pointers
        let mut raw_ptrs: Vec<*const c_char> = Vec::with_capacity(yad_ref.rows.len());

        // Convert each row name to CString and take ownership of its raw pointer
        for (row_name, _) in &yad_ref.rows {
            let cstring = CString::new(row_name.as_str()).unwrap();
            raw_ptrs.push(cstring.into_raw());
        }

        // Convert the Vec into a boxed slice on the heap and return its raw pointer
        let boxed_slice = raw_ptrs.into_boxed_slice();
        Box::into_raw(boxed_slice) as *mut *const c_char
    }
}

/// Frees an array of C string pointers previously returned by `yad_rows_names`,
/// including each individual string.
///
/// # Safety
/// - The caller must ensure that `names` was obtained from `yad_rows_names`.
/// - After calling this function, `names` and its strings must not be used again.
///
/// # Notes
/// - This function safely reconstructs each CString from its raw pointer and frees it.
/// - Finally, the array of pointers itself is deallocated.
#[unsafe(no_mangle)]
pub extern "C" fn yad_rows_names_free(names: *mut *const c_char, len: usize) {
    if names.is_null() {
        return;
    }

    unsafe {
        // Reconstruct a slice from the raw pointer
        let slice = std::slice::from_raw_parts_mut(names as *mut *mut c_char, len);

        // Free each individual CString
        for &mut ptr in slice.as_mut() {
            if !ptr.is_null() {
                let _ = CString::from_raw(ptr);
            }
        }

        // Free the array of pointers itself
        let _ = Box::from_raw(slice as *mut [_] as *mut *mut c_char);
    }
}


/// Serializes a `YAD` into a raw byte buffer.
///
/// # Parameters
/// - `yad`: Pointer to a `YAD`.
///
/// # Returns
/// - Pointer to a buffer containing serialized `YAD`, or null on error.
/// - The buffer is heap-allocated and must remain valid until manually freed.
#[unsafe(no_mangle)]
pub extern "C" fn yad_as_buffer(yad: *mut YAD) -> *const u8 {
    if yad.is_null() {
        return std::ptr::null();
    }

    unsafe {
        match serialize(&*yad) {
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

/// Deserializes a `YAD` from a raw byte buffer.
///
/// # Parameters
/// - `buff`: Pointer to a buffer containing serialized `YAD`.
/// - `len`: Length of the buffer.
///
/// # Returns
/// - Pointer to a heap-allocated `YAD`, or null if deserialization fails.
#[unsafe(no_mangle)]
pub extern "C" fn yad_from_buffer(buff: *const u8, len: usize) -> *mut YAD {
    if buff.is_null() || len == 0 {
        return std::ptr::null_mut();
    }

    unsafe {
        let slice = std::slice::from_raw_parts(buff, len);
        let vec = slice.to_vec();
        match deserialize(vec) {
            Ok(yad) => Box::into_raw(Box::new(yad)),
            Err(_) => std::ptr::null_mut(),
        }
    }
}
