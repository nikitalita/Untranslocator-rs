#![cfg(target_os = "macos")]

use std::path::Path;

#[test]
fn integration_non_translocated_path_roundtrip() {
    let input = Path::new("/Applications");
    let resolved = untranslocator::resolve_translocated_path(input)
        .expect("non-translocated path resolution should succeed");
    assert_eq!(resolved, input);
}
