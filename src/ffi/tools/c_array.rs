use crate::core::Value;

/// A C-compatible wrapper around a Rust `Vec<Value>`
///
/// This struct is intended to be used across FFI boundaries.
/// The layout is compatible with C (`#[repr(C)]`), and it exposes
/// a raw pointer, length, and capacity.
///
/// # Fields
/// - `ptr`: Pointer to the first element in the array. Can be null if empty.
/// - `len`: Number of valid elements in the array.
/// - `cap`: Allocated capacity of the array.
///
/// # Safety
/// Accessing `ptr` directly is unsafe. Always use the provided FFI functions
/// to manipulate the array safely.
#[repr(C)]
pub struct CArray {
    pub ptr: *mut Value,  // raw pointer to array elements
    pub len: usize,       // number of valid elements
    pub cap: usize,       // allocated capacity
}

/// Creates a new empty `CArray`.
///
/// # Returns
/// - A raw pointer to a heap-allocated `CArray`.
///
/// # Safety
/// - The returned pointer must eventually be freed using `free_c_array` to prevent memory leaks.
#[unsafe(no_mangle)]
pub extern "C" fn c_array_new() -> *mut CArray {
    let mut vec: Vec<Value> = Vec::new();
    let arr = CArray {
        ptr: vec.as_mut_ptr(),
        len: vec.len(),
        cap: vec.capacity(),
    };
    std::mem::forget(vec); // Prevent Rust from deallocating
    Box::into_raw(Box::new(arr))
}

/// Inserts a `Value` into the `CArray` at the specified index.
///
/// # Parameters
/// - `arr`: Pointer to the `CArray`.
/// - `index`: The position to insert the value.
/// - `value`: Pointer to the `Value` to insert.
///
/// # Returns
/// - `true` if insertion succeeded.
/// - `false` if the array pointer or value pointer is null, or if the index is out of bounds.
///
/// # Safety
/// - Both `arr` and `value` must be valid, non-null pointers.
#[unsafe(no_mangle)]
pub extern "C" fn c_array_insert(arr: *mut CArray, index: usize, value: *mut Value) -> bool {
    if arr.is_null() || value.is_null() {
        return false;
    }
    unsafe {
        let mut vec = Vec::from_raw_parts((*arr).ptr, (*arr).len, (*arr).cap);
        if index > vec.len() {
            let arr_mut = &mut *arr;
            arr_mut.ptr = vec.as_mut_ptr();
            arr_mut.len = vec.len();
            arr_mut.cap = vec.capacity();
            std::mem::forget(vec);
            return false;
        }

        let val: Value = *Box::from_raw(value);
        vec.insert(index, val);

        let arr_mut = &mut *arr;
        arr_mut.ptr = vec.as_mut_ptr();
        arr_mut.len = vec.len();
        arr_mut.cap = vec.capacity();
        std::mem::forget(vec);
    }
    true
}

/// Retrieves a value from the `CArray` at the specified index.
///
/// # Parameters
/// - `arr`: Pointer to the `CArray`.
/// - `index`: Index of the element to retrieve.
/// - `out`: Pointer to a `Value` where the result will be written.
///
/// # Returns
/// - `true` if the value was successfully retrieved.
/// - `false` if the index is out of bounds or pointers are null.
///
/// # Safety
/// - `arr` and `out` must be valid, non-null pointers.
#[unsafe(no_mangle)]
pub extern "C" fn c_array_get(arr: *mut CArray, index: usize, out: *mut Value) -> bool {
    if arr.is_null() || out.is_null() {
        return false;
    }
    unsafe {
        let vec = Vec::from_raw_parts((*arr).ptr, (*arr).len, (*arr).cap);
        let result = if index < vec.len() {
            std::ptr::write(out, vec[index].clone());
            true
        } else {
            false
        };
        let arr_mut = &mut *arr;
        arr_mut.ptr = vec.as_ptr() as *mut Value;
        arr_mut.len = vec.len();
        arr_mut.cap = vec.capacity();
        std::mem::forget(vec);
        result
    }
}

/// Removes a value from the `CArray` at the specified index.
///
/// # Parameters
/// - `arr`: Pointer to the `CArray`.
/// - `index`: Index of the element to remove.
/// - `out`: Optional pointer to a `Value` where the removed element will be written.
///
/// # Returns
/// - `true` if removal succeeded.
/// - `false` if the index is out of bounds or `arr` is null.
///
/// # Safety
/// - `arr` must be a valid pointer.
/// - `out` can be null if the removed value does not need to be retrieved.
#[unsafe(no_mangle)]
pub extern "C" fn c_array_remove(arr: *mut CArray, index: usize, out: *mut Value) -> bool {
    if arr.is_null() {
        return false;
    }
    unsafe {
        let mut vec = Vec::from_raw_parts((*arr).ptr, (*arr).len, (*arr).cap);
        if index >= vec.len() {
            let arr_mut = &mut *arr;
            arr_mut.ptr = vec.as_mut_ptr();
            arr_mut.len = vec.len();
            arr_mut.cap = vec.capacity();
            std::mem::forget(vec);
            return false;
        }

        let removed = vec.remove(index);
        if !out.is_null() {
            std::ptr::write(out, removed.clone());
        }

        let arr_mut = &mut *arr;
        arr_mut.ptr = vec.as_mut_ptr();
        arr_mut.len = vec.len();
        arr_mut.cap = vec.capacity();
        std::mem::forget(vec);
    }
    true
}

/// Returns the number of elements in the `CArray`.
///
/// # Parameters
/// - `arr`: Pointer to the `CArray`.
///
/// # Returns
/// - Length of the array, or 0 if the pointer is null.
#[unsafe(no_mangle)]
pub extern "C" fn c_array_length(arr: *mut CArray) -> usize {
    if arr.is_null() {
        return 0;
    }
    unsafe { (*arr).len }
}

/// Returns the capacity of the `CArray`.
///
/// # Parameters
/// - `arr`: Pointer to the `CArray`.
///
/// # Returns
/// - Capacity of the array, or 0 if the pointer is null.
#[unsafe(no_mangle)]
pub extern "C" fn c_array_capacity(arr: *mut CArray) -> usize {
    if arr.is_null() {
        return 0;
    }
    unsafe { (*arr).cap }
}

/// Frees a `CArray` and its underlying memory.
///
/// # Parameters
/// - `arr`: Pointer to the `CArray` to free.
///
/// # Safety
/// - `arr` must be a pointer previously returned by `c_array_new`.
/// - After calling this function, `arr` must not be used again.
#[unsafe(no_mangle)]
pub extern "C" fn free_c_array(arr: *mut CArray) {
    if arr.is_null() {
        return;
    }
    unsafe {
        let c_arr = Box::from_raw(arr);

        if !c_arr.ptr.is_null() {
            drop(Vec::from_raw_parts(c_arr.ptr, c_arr.len, c_arr.cap));
        }
    }
}
