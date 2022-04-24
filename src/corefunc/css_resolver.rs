use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
pub fn is_css_file<P: AsRef<Path>>(path: P) -> bool {
    let suffix = path.as_ref().extension();
    let suffix_str= suffix.map(|s|s.to_str());
    suffix_str == Some(Some("css"))
}
pub fn walk_css_dir<P: AsRef<Path>>(folder: &P) -> io::Result<Option<Vec<Box<PathBuf>>>> {
    let path = folder.as_ref();
    let pathbuf = path.to_path_buf();
    let mut res_vec = vec![];
    if let Some(name) = pathbuf.iter().last() {
        if name == OsStr::new("css") && path.is_dir() {
            if let Ok(entrys) = fs::read_dir(path) {
                for entry in entrys {
                    let css_file = entry?.path();
                    let css_file_out=css_file.clone();
                    if is_css_file(css_file) {
                        res_vec.push(Box::new(css_file_out));
                    }
                }
                Ok(Some(res_vec))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
