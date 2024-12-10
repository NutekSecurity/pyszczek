use std::ffi::CString;
use std::os::raw::c_char;

/// Return a string with a cat face
/// The caller is responsible for freeing the memory
/// using the `free_string` function
///
/// # Safety
/// The caller must ensure that the returned pointer is freed
/// using the `free_string` function
///
#[no_mangle]
pub extern "C" fn nutek() -> *const c_char {
    let pyszczek = r#"
 /\_/\  |
( o.o ) o
 > ^ <
"#;
    // .to_string();
    let s = CString::new(pyszczek).unwrap();
    s.into_raw()
}

/// Optional: Function to free the memory (call this from Ruby when done)
#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if !s.is_null() {
            let _ = CString::from_raw(s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nutek() {
        let result = nutek();
        assert!(!result.is_null());
        unsafe {
            let cs = CString::from_raw(result as *mut c_char);
            let s = cs.into_string().unwrap();
            assert_eq!(
                s,
                r#"
 /\_/\  |
( o.o ) o
 > ^ <
"#
            );
        }
    }
}
