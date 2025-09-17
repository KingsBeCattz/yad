pub mod key;
pub mod row;

use crate::{YAD, Version, Row};
use std::ffi::{CStr, CString};
use std::ptr;

/// # Version FFI (C ABI)
///
/// Provides functions to create, serialize, and deserialize `Version` objects
/// for use from C code. All functions use `#[unsafe(no_mangle)]`.

/// Creates a new [`Version`] from individual fields.
///
/// # Parameters
/// - `major`: Major version number (breaking changes)
/// - `minor`: Minor version number (new features)
/// - `patch`: Patch version number (bug fixes)
/// - `beta`: Beta/pre-release identifier (0 = stable)
///
/// # Returns
/// - Pointer to a newly allocated [`Version`]. Must be freed using `version_free`.
#[unsafe(no_mangle)]
pub extern "C" fn version_new(major: u8, minor: u8, patch: u8, beta: u8) -> *mut Version {
    Box::into_raw(Box::new(Version { major, minor, patch, beta }))
}

/// Frees a [`Version`] previously allocated by `version_new`.
///
/// # Safety
/// - `version` must be a valid pointer returned by `version_new`.
#[unsafe(no_mangle)]
pub extern "C" fn version_free(version: *mut Version) {
    unsafe { if !version.is_null() { let _ = Box::from_raw(version); } }
}

/// Serializes a [`Version`] into a 5-byte array.
///
/// # Safety
/// - `version` must be a valid pointer to a [`Version`].
/// - `out_bytes` must point to at least 5 writable bytes.
#[unsafe(no_mangle)]
pub extern "C" fn version_serialize(version: *const Version, out_bytes: *mut u8) {
    unsafe {
        if version.is_null() || out_bytes.is_null() { return; }
        let bytes = (*version).serialize();
        ptr::copy_nonoverlapping(bytes.as_ptr(), out_bytes, bytes.len());
    }
}

/// Deserializes a [`Version`] from a 5-byte buffer.
///
/// # Safety
/// - `bytes` must point to at least 5 bytes.
/// - Returns null pointer on failure.
/// - Allocated memory must be freed with `version_free`.
#[unsafe(no_mangle)]
pub extern "C" fn version_deserialize(bytes: *const u8) -> *mut Version {
    unsafe {
        if bytes.is_null() { return ptr::null_mut(); }
        let slice = std::slice::from_raw_parts(bytes, 5).to_vec();
        match Version::deserialize(slice) {
            Ok(ver) => Box::into_raw(Box::new(ver)),
            Err(_) => ptr::null_mut(),
        }
    }
}

/// # YAD FFI (C ABI)
///
/// Provides functions to create, manipulate, serialize, and deserialize `YAD` documents
/// for use from C code.

/// Creates a new empty [`YAD`] document with the specified version.
///
/// # Safety
/// - `version` must be a valid pointer to a [`Version`].
/// - Returns a pointer to a new [`YAD`] object. Must be freed with `yad_free`.
#[unsafe(no_mangle)]
pub extern "C" fn yad_new_empty(version: *const Version) -> *mut YAD {
    unsafe {
        if version.is_null() { return ptr::null_mut(); }
        Box::into_raw(Box::new(YAD::new_empty((*version).clone())))
    }
}

/// Creates a new [`YAD`] document from version and an array of [`Row`] pointers.
///
/// # Safety
/// - `version` must be a valid pointer to [`Version`].
/// - `rows` is an array of `*mut Row` of length `rows_len`.
/// - Null pointers inside `rows` are ignored.
/// - Returns a pointer to a new [`YAD`] object. Must be freed with `yad_free`.
#[unsafe(no_mangle)]
pub extern "C" fn yad_new(version: *const Version, rows: *const *mut Row, rows_len: usize) -> *mut YAD {
    unsafe {
        if version.is_null() { return ptr::null_mut(); }
        let mut vec_rows = Vec::with_capacity(rows_len);
        if !rows.is_null() {
            for i in 0..rows_len {
                let row_ptr = *rows.add(i);
                if !row_ptr.is_null() { vec_rows.push((*row_ptr).clone()); }
            }
        }
        Box::into_raw(Box::new(YAD::new((*version).clone(), vec_rows)))
    }
}

/// Frees a [`YAD`] object previously allocated.
///
/// # Safety
/// - `yad` must be a valid pointer returned by `yad_new` or `yad_new_empty`.
#[unsafe(no_mangle)]
pub extern "C" fn yad_free(yad: *mut YAD) {
    unsafe { if !yad.is_null() { let _ = Box::from_raw(yad); } }
}

/// Inserts a [`Row`] into the [`YAD`] document.
///
/// # Safety
/// - `yad` must be a valid pointer to a [`YAD`].
/// - `row` must be a valid pointer to a [`Row`].
#[unsafe(no_mangle)]
pub extern "C" fn yad_insert_row(yad: *mut YAD, row: *mut Row) {
    unsafe {
        if yad.is_null() || row.is_null() { return; }
        let yad = &mut *yad;
        let row = &*row;
        yad.rows.insert(row.name.clone(), row.clone());
    }
}

/// Removes a [`Row`] from the [`YAD`] document by name.
///
/// # Safety
/// - `yad` must be a valid pointer to a [`YAD`].
/// - `name` must be a null-terminated C string.
///
/// # Returns
/// - Pointer to removed [`Row`] or null if not found.
/// - Caller must free with `row_free`.
#[unsafe(no_mangle)]
pub extern "C" fn yad_remove_row(yad: *mut YAD, name: *const i8) -> *mut Row {
    unsafe {
        if yad.is_null() || name.is_null() { return ptr::null_mut(); }
        let cstr = match CStr::from_ptr(name).to_str() { Ok(s) => s, Err(_) => return ptr::null_mut() };
        match (*yad).rows.remove(cstr) {
            Some(row) => Box::into_raw(Box::new(row)),
            None => ptr::null_mut(),
        }
    }
}

/// Serializes a [`YAD`] document into a byte buffer.
///
/// # Safety
/// - `yad` must be a valid pointer to a [`YAD`].
/// - `out_bytes` must point to a valid buffer of at least `max_len` bytes.
/// - Returns the number of bytes written.
#[unsafe(no_mangle)]
pub extern "C" fn yad_serialize(yad: *const YAD, out_bytes: *mut u8, max_len: usize) -> usize {
    unsafe {
        if yad.is_null() || out_bytes.is_null() { return 0; }
        let yad = &*yad;
        match yad.serialize() {
            Ok(vec) => {
                let len = vec.len().min(max_len);
                ptr::copy_nonoverlapping(vec.as_ptr(), out_bytes, len);
                len
            }
            Err(_) => 0,
        }
    }
}

/// Deserializes a [`YAD`] document from a byte buffer.
///
/// # Safety
/// - `bytes` must point to a valid buffer of length `len`.
/// - Returns null on failure. Allocated memory must be freed with `yad_free`.
#[unsafe(no_mangle)]
pub extern "C" fn yad_deserialize(bytes: *const u8, len: usize) -> *mut YAD {
    unsafe {
        if bytes.is_null() || len == 0 { return ptr::null_mut(); }
        let vec = std::slice::from_raw_parts(bytes, len).to_vec();
        match YAD::deserialize(vec) {
            Ok(yad) => Box::into_raw(Box::new(yad)),
            Err(_) => ptr::null_mut(),
        }
    }
}

/// # YAD FFI – Row Accessors
///
/// These functions provide C-compatible access to rows within a YAD document.

/// Retrieves a [`Row`] from a [`YAD`] by name.
///
/// # Safety
/// - `yad` must be a valid pointer to a [`YAD`].
/// - `name` must be a null-terminated C string.
/// - Returns a pointer to a cloned [`Row`], or null if not found.
/// - Caller must free the returned row using `row_free`.
#[unsafe(no_mangle)]
pub extern "C" fn yad_get_row(yad: *const YAD, name: *const i8) -> *mut Row {
    unsafe {
        if yad.is_null() || name.is_null() { return ptr::null_mut(); }
        let cstr = match CStr::from_ptr(name).to_str() { Ok(s) => s, Err(_) => return ptr::null_mut() };
        match (*yad).rows.get(cstr) {
            Some(row) => Box::into_raw(Box::new(row.clone())),
            None => ptr::null_mut(),
        }
    }
}

/// Removes a [`Row`] from a [`YAD`] by name.
///
/// # Safety
/// - `yad` must be a valid pointer to a [`YAD`].
/// - `name` must be a null-terminated C string.
/// - Returns a pointer to the removed [`Row`] or null if not found.
/// - Caller must free the returned row using `row_free`.
#[unsafe(no_mangle)]
pub extern "C" fn yad_remove_row_by_name(yad: *mut YAD, name: *const i8) -> *mut Row {
    unsafe {
        if yad.is_null() || name.is_null() { return ptr::null_mut(); }
        let cstr = match CStr::from_ptr(name).to_str() { Ok(s) => s, Err(_) => return ptr::null_mut() };
        match (*yad).rows.remove(cstr) {
            Some(row) => Box::into_raw(Box::new(row)),
            None => ptr::null_mut(),
        }
    }
}

/// Sets or replaces a [`Row`] in the [`YAD`] document.
///
/// # Safety
/// - `yad` must be a valid pointer to a [`YAD`].
/// - `row` must be a valid pointer to a [`Row`].
/// - The row will be cloned into the document; original memory must still be freed separately if needed.
#[unsafe(no_mangle)]
pub extern "C" fn yad_set_row(yad: *mut YAD, row: *mut Row) {
    unsafe {
        if yad.is_null() || row.is_null() { return; }
        let yad = &mut *yad;
        let row = &*row;
        yad.rows.insert(row.name.clone(), row.clone());
    }
}

/// # YAD FFI – Row Utilities
///
/// Additional helper functions for row management in a C-compatible manner.

/// Returns the number of rows in the YAD document.
///
/// # Safety
/// - `yad` must be a valid pointer to a [`YAD`] document.
#[unsafe(no_mangle)]
pub extern "C" fn yad_row_count(yad: *const YAD) -> usize {
    unsafe {
        if yad.is_null() {
            return 0;
        }
        (*yad).rows.len()
    }
}

/// Returns a heap-allocated array of C strings representing the row names.
///
/// # Safety
/// - `yad` must be a valid pointer to a [`YAD`] document.
/// - Returns a pointer to an array of `*const i8` (C strings).
/// - Caller is responsible for freeing each string with `CString::from_raw`
///   and the array itself with `Box::from_raw`.
#[unsafe(no_mangle)]
pub extern "C" fn yad_row_names(yad: *const YAD) -> *mut *mut i8 {
    unsafe {
        if yad.is_null() {
            return ptr::null_mut();
        }

        let yad = &*yad;
        let mut cstrings: Vec<*mut i8> = Vec::with_capacity(yad.rows.len());

        for row_name in yad.rows.keys() {
            let cstr = CString::new(row_name.as_str()).unwrap_or_else(|_| CString::new("").unwrap());
            cstrings.push(cstr.into_raw());
        }

        let ptr_array = cstrings.into_boxed_slice();
        Box::into_raw(ptr_array) as *mut *mut i8
    }
}

/// Frees the array of C strings returned by [`yad_row_names`].
///
/// # Safety
/// - `names` must be a pointer returned by [`yad_row_names`].
/// - `count` must be the number of elements in the array.
#[unsafe(no_mangle)]
pub extern "C" fn yad_row_names_free(names: *mut *mut i8, count: usize) {
    unsafe {
        if names.is_null() {
            return;
        }

        let names_slice = std::slice::from_raw_parts_mut(names, count);

        for &mut name_ptr in names_slice {
            if !name_ptr.is_null() {
                // Reclaim CString memory
                let _ = CString::from_raw(name_ptr);
            }
        }
    }
}
