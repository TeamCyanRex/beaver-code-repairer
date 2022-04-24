
use super::css_resolver::*;
#[test]
fn path_name_test() {
    use std::ffi::OsStr;
    use std::path::PathBuf;
    let pathbuf: PathBuf = "/test/css".into();
    assert_eq!(pathbuf.iter().last(), Some(OsStr::new("css")));
}
#[test]
fn css_name_test() {
    use std::path::Path;

    let css_tar = Path::new("t.css");
    assert!(is_css_file(css_tar));
}
