use super::css_resolver::*;
use std::path::Path;
#[test]
fn path_name_test() {
    use std::ffi::OsStr;
    use std::path::PathBuf;
    let pathbuf: PathBuf = "/test/css".into();
    assert_eq!(pathbuf.iter().last(), Some(OsStr::new("css")));
}
#[test]
fn is_css_file_test() {
    use std::path::Path;

    let css_tar = Path::new(r"dist\dark-c5d600b3bdd64443.css");
    assert!(is_css_file(css_tar));
}
#[test]
#[ignore]
fn walk_css_dir_test() {
    let res = walk_css_dir(Path::new(r"css")).unwrap();
    assert_eq!(res.map(|v| v.len()), Some(5));
}
#[test]
#[ignore]
fn walk_dir_for_css_test() {
    let res = walk_dir_for_css(Path::new(r"dist")).unwrap();
    assert_eq!(res.map(|v| v.len()), Some(5));
}

#[test]
#[ignore]
fn extract_css_basic_class_test() {
    let test1 = Path::new(r"test-material\test1.css");
    let test2 = Path::new(r"test-material\test2.css");
    let test3 = Path::new(r"test-material\test3.css");
    let res1 = extract_css_basic_class(test1);
    let res2 = extract_css_basic_class(test2);
    let res3 = extract_css_basic_class(test3);
    println!("{:?}", res1);
    println!("{:?}", res2);
    println!("{:?}", res3);
    assert_eq!(
        res1,
        Some(vec![
            "color".to_owned(),
            "button".to_owned(),
            "a".to_owned(),
            "b".to_owned()
        ])
    );
    assert_eq!(res2, Some(vec!["kkk".to_owned()]));
    assert_eq!(res3, None);
}
use regex::Regex;
#[test]
fn css_class_regex_test() {
    let tar = r".kkk-button {";

    let pattern_res = Regex::new(r"\..+\-([0-9[:alpha:]\-_]+)\s*\{").unwrap();
    assert_eq!(
        pattern_res.captures(tar).unwrap().get(1).unwrap().as_str(),
        "button"
    );
}
#[test]
fn renderlist_test() {
    let res = CssRenderList::new(Path::new(r"css"));
    println!("{:#?}", res);
}
