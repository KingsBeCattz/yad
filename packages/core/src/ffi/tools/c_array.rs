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

/// Pushes a new `Value` into a `CArray`.
///
/// # Parameters
/// - `arr`: A raw pointer to a mutable [`CArray`].
/// - `value`: A raw pointer to a heap-allocated [`Value`].
///
/// # Returns
/// - `true` if the operation succeeded.
/// - `false` if:
///   - Either pointer is null.
///   - The array has reached `isize::MAX` capacity (overflow safeguard).
///
/// # Behavior
/// - Takes ownership of the `value` pointer (frees the original `Box`).
/// - Converts the internal raw pointer of the `CArray` into a temporary [`Vec<Value>`].
/// - If the vector is at capacity, calls [`Vec::reserve`] to allocate more space.
/// - Pushes the new value into the vector.
/// - Updates the `ptr`, `len`, and `cap` fields of the `CArray` with the new vector state.
/// - Calls [`std::mem::forget`] to prevent the temporary `Vec` from freeing its buffer,
///   since ownership is transferred back to the `CArray`.
///
/// # Safety
/// - `arr` must be a valid, non-null pointer to a properly initialized `CArray`.
/// - `value` must be a valid, non-null pointer to a heap-allocated `Value`.
/// - After this call, the caller must not use the original `value` pointer again,
///   as its ownership has been transferred.
/// - Misuse can lead to undefined behavior, memory leaks, or double frees.
///
/// # Example (pseudo-usage in C/FFI)
/// ```c
/// CArray* arr = c_array_new();
/// Value* val = value_from_int(42);
/// bool ok = c_array_push(arr, val);
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn c_array_push(arr: *mut CArray, value: *mut Value) -> bool {
    if arr.is_null() || value.is_null() {
        return false;
    }
    unsafe {
        // Prevent exceeding maximum addressable size
        if (*arr).cap >= isize::MAX as usize {
            return false;
        }

        // Rebuild Vec<Value> from raw parts
        let mut vec = Vec::from_raw_parts((*arr).ptr, (*arr).len, (*arr).cap);

        // Take ownership of the Value pointer
        let val: Value = *Box::from_raw(value);

        // Grow capacity if full
        if vec.len() == vec.capacity() {
            vec.reserve(1);
        }
        vec.push(val);

        // Update array metadata
        let arr_mut = &mut *arr;
        arr_mut.ptr = vec.as_mut_ptr();
        arr_mut.len = vec.len();
        arr_mut.cap = vec.capacity();

        // Prevent Vec from freeing its buffer
        std::mem::forget(vec);
    }
    true
}


/// Returns a heap-allocated clone of the `Value` stored in the `CArray` at `index`.
///
/// This function does **not** expose a direct pointer into the internal buffer. Instead,
/// it clones the element and returns ownership of a newly allocated `Value*` to the caller.
/// The caller is responsible for destroying that `Value*` with this library's designated
/// deallocator (e.g., `value_free`).
///
/// # Parameters
/// - `arr`: Pointer to a valid `CArray` whose internal buffer was created by this library.
/// - `index`: Zero-based index of the element to retrieve.
///
/// # Returns
/// - On success: a non-null `*mut Value` pointing to a freshly allocated clone of the element.
/// - On failure (null `arr`, out-of-bounds index, or panic during clone): `null_mut()`.
///
/// # Complexity
/// - O(1) for indexing, plus the cost of `Value::clone()`.
///
/// # Ownership & Lifetime
/// - The returned pointer owns its storage and must be freed by the caller using the
///   appropriate destructor from this library.
/// - This function does **not** transfer or modify ownership of the underlying `CArray` buffer.
///
/// # Safety
/// - `arr` must be a non-null pointer to a well-formed `CArray` with fields (`ptr`, `len`, `cap`)
///   describing a buffer allocated by the same Rust allocator.
/// - No other thread may mutate the `CArray` concurrently while this function executes.
/// - `Value` must implement `Clone` and cloning must not unwind across the FFI boundary.
/// - Invoking this function with an invalid `arr` or corrupted invariants constitutes undefined behavior.
///
/// # Notes
/// - The implementation now avoids temporary ownership of the buffer by using
///   `slice::from_raw_parts` instead of `Vec::from_raw_parts`.
/// - The body is wrapped in `catch_unwind` to prevent panics from propagating
///   across the FFI boundary.
#[unsafe(no_mangle)]
pub extern "C" fn c_array_get(arr: *mut CArray, index: usize) -> *mut Value {
    if arr.is_null() {
        return std::ptr::null_mut();
    }

    let result = std::panic::catch_unwind(|| unsafe {
        let arr_ref = &*arr;

        debug_assert!(arr_ref.len <= arr_ref.cap);
        debug_assert!(arr_ref.ptr.is_null() == (arr_ref.len == 0));

        let slice = std::slice::from_raw_parts(arr_ref.ptr, arr_ref.len);

        if let Some(v) = slice.get(index) {
            Box::into_raw(Box::new(v.clone()))
        } else {
            std::ptr::null_mut()
        }
    });

    result.unwrap_or_else(|_| std::ptr::null_mut())
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

/// Returns a raw pointer to the internal buffer of the CArray and its length.
///
/// # Parameters
/// - `arr`: Pointer to a valid CArray.
///
/// # Returns
/// - Tuple `(ptr, len)` as `(Value**, usize)`; returns `(null_mut(), 0)` if `arr` is null.
///
/// # Safety
/// - The caller must not mutate the returned pointers.
/// - The caller does not own the Values; they are still owned by the CArray.
/// - Thread-safety: the CArray must not be mutated concurrently.
#[unsafe(no_mangle)]
pub extern "C" fn c_array_as_ptr(arr: *mut CArray, out_len: *mut usize) -> *mut *mut Value {
    if arr.is_null() || out_len.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        *out_len = (*arr).len;
        (*arr).ptr as *mut *mut Value
    }
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
