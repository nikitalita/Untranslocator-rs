//! Rust bindings for resolving paths affected by macOS App Translocation.
//!
//! This crate wraps the Objective-C implementation from `Untranslocator.Lib`
//! and exposes a small Rust-friendly API.

use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

#[cfg(not(target_os = "macos"))]
compile_error!("The untranslocator crate only supports macOS targets.");

/// Raw native bindings.
pub mod ffi {
    use std::os::raw::c_char;

    unsafe extern "C" {
        pub fn untranslocator_resolve_path(path_utf8: *const c_char) -> *const c_char;
        pub fn untranslocator_free_string(owned_str: *const c_char);
    }
}

/// Error returned when a path cannot be resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveError {
    /// The input path is not valid UTF-8.
    InputNotUtf8,
    /// The input path contains an interior null byte.
    InputContainsNul,
    /// Native resolver returned no value.
    NativeNull,
    /// Native resolver returned non-UTF-8 output.
    NativeNotUtf8,
}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputNotUtf8 => f.write_str("input path is not valid UTF-8"),
            Self::InputContainsNul => f.write_str("input path contains an interior NUL byte"),
            Self::NativeNull => f.write_str("native resolver returned a null pointer"),
            Self::NativeNotUtf8 => f.write_str("native resolver returned non-UTF-8 data"),
        }
    }
}

impl std::error::Error for ResolveError {}

/// Resolves the original path for a potentially translocated path.
pub fn resolve_translocated_path<P>(path: P) -> Result<PathBuf, ResolveError>
where
    P: AsRef<Path>,
{
    let path_str = path.as_ref().to_str().ok_or(ResolveError::InputNotUtf8)?;
    let c_input = CString::new(path_str).map_err(|_| ResolveError::InputContainsNul)?;

    let raw_ptr = unsafe { ffi::untranslocator_resolve_path(c_input.as_ptr() as *const c_char) };
    if raw_ptr.is_null() {
        return Err(ResolveError::NativeNull);
    }

    let output_result = unsafe {
        let value = CStr::from_ptr(raw_ptr);
        value
            .to_str()
            .map(PathBuf::from)
            .map_err(|_| ResolveError::NativeNotUtf8)
    };
    unsafe { ffi::untranslocator_free_string(raw_ptr) };

    output_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_same_non_translocated_path() {
        let input = Path::new("/tmp/untranslocator_test.app");
        let resolved = resolve_translocated_path(input).expect("path resolution should succeed");
        assert_eq!(resolved, input);
    }

    #[test]
    fn accepts_pathbuf_string_and_str() {
        let input_buf = PathBuf::from("/tmp/untranslocator_test.app");
        let from_pathbuf =
            resolve_translocated_path(input_buf.clone()).expect("PathBuf input should work");
        assert_eq!(from_pathbuf, input_buf);

        let input_string = String::from("/tmp/untranslocator_test.app");
        let from_string =
            resolve_translocated_path(input_string.clone()).expect("String input should work");
        assert_eq!(from_string, PathBuf::from(input_string));

        let from_str = resolve_translocated_path("/tmp/untranslocator_test.app")
            .expect("&str input should work");
        assert_eq!(from_str, PathBuf::from("/tmp/untranslocator_test.app"));
    }

    #[test]
    fn display_messages_are_stable() {
        assert_eq!(
            ResolveError::InputContainsNul.to_string(),
            "input path contains an interior NUL byte"
        );
    }
}
