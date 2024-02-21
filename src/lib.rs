#![allow(bad_style)]
extern crate libc;

pub mod netmap;
pub mod netmap_user;
mod netmap_util;

#[cfg(feature = "netmap_with_libs")]
pub mod netmap_with_libs;

#[cfg(feature = "libnetmap")]
pub mod libnetmap;

#[cfg(feature = "libnetmap")]
#[cfg(test)]
mod libnetmap_tests {
    use std::ffi::CString;
    use libnetmap::{nmport_new, nmport_parse};

    #[test]
    fn check_linking() {
        unsafe {
            let d = nmport_new();
            let portspec = CString::new("netmap:bond}0@1").unwrap();
            let status = nmport_parse(d, portspec.as_ptr());
            assert_eq!(0, status);
        }
    }
}
