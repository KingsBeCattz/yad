mod yad;
pub use yad::*;
pub mod value;
pub mod row;
pub mod key;
pub mod tools;

/// Frees a heap-allocated buffer previously returned from Rust.
///
/// # Parameters
/// - `ptr`: Pointer to the buffer to free.
///
/// # Safety
/// - `ptr` must be a pointer previously returned from Rust (e.g., a buffer from `yad_as_buffer` or `row_as_buffer`) or null.
/// - After calling this function, `ptr` must not be used again to avoid undefined behavior.
///
/// # Example
/// ```c
/// const uint8_t* buf = yad_as_buffer(yad);
/// // ... use the buffer ...
/// free_buffer((uint8_t*)buf);
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn free_buffer(ptr: *mut u8) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}


/* Author: Johan | Date: 8/21/2025 11:15 PM
- Key functions
- Doc all functions lol
- Test lol
*/