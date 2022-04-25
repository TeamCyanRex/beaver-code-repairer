use regex::Regex;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::path::PathBuf;
pub fn is_css_file<P: AsRef<Path>>(path: P) -> bool {
    let suffix = path.as_ref().extension();
    let suffix_str = suffix.map(|s| s.to_str());
    suffix_str == Some(Some("css"))
}
pub fn walk_css_dir<P: AsRef<Path>>(folder: P) -> io::Result<Option<Vec<Box<PathBuf>>>> {
    let path = folder.as_ref();
    let pathbuf = path.to_path_buf();
    let mut res_vec = vec![];
    if let Some(name) = pathbuf.iter().last() {
        if name == OsStr::new("css") && path.is_dir() {
            if let Ok(entrys) = fs::read_dir(path) {
                for entry in entrys {
                    let css_file = entry?.path();
                    let css_file_out = css_file.clone();
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
pub fn extract_css_basic_class<P: AsRef<Path>>(path: P) -> Option<Vec<String>> {
    let mut classes_res = fs::read_to_string(path);
    if let Ok(classes) = classes_res {
        let pattern_res = Regex::new(r"\..+\-([0-9[:alpha:]\-_]+)\s*\{");
        if let Ok(pattern) = pattern_res {
            let mut cursor = io::Cursor::new(classes);
            let mut buffer = String::new();
            let mut res = vec![];
            while let Ok(num) = cursor.read_line(&mut buffer) {
                if num == 0 {
                    break;
                }
                let class_name_opt = pattern.captures(&buffer);
                if let Some(class_name) = class_name_opt {
                    res.push(class_name[1].to_string());
                }
                buffer.clear()
            }
            Some(res)
        } else {
            None
        }
    } else {
        None
    }
}
fn cross_vec_str(
    main_key: &Vec<String>,
    sub_key: &Vec<String>,
) -> HashMap<String, HashMap<String, String>> {
    let mut res = HashMap::new();
    for main in main_key.iter() {
        let mut subs = HashMap::new();
        for sub in sub_key.iter() {
            subs.insert(sub.clone(), main.to_owned() + "-" + sub);
        }
        res.insert(main.clone(), subs);
    }
    res
}

#[derive(Debug, Default)]
pub struct CssRenderList {
    pub themelist: Vec<String>,
    widgitlist: Vec<String>,
    pub renderlist: HashMap<String, HashMap<String, String>>,
}
impl CssRenderList {
    pub(crate) fn new<P: AsRef<Path>>(path: P) -> Self {
        if let Ok(Some(vec_csses)) = walk_css_dir(path) {
            if let Some(first_css) = vec_csses.first() {
                let mut widgitlist = vec![];
                let mut themelist = vec![];
                let widgets: Vec<String> = match extract_css_basic_class(first_css.as_ref()) {
                    Some(w) => w,
                    _ => return Default::default(),
                };
                widgets.into_iter().for_each(|s| widgitlist.push(s));

                for css in vec_csses.iter() {
                    let os_str = css.file_name().map(|s| match PathBuf::from(s).file_stem() {
                        Some(s) => s.to_os_string(),
                        _ => return Default::default(),
                    });
                    let theme_opt = os_str.map(|s| match s.to_str() {
                        Some(s) => s.to_string(),
                        _ => return Default::default(),
                    });
                    if let Some(theme) = theme_opt {
                        themelist.push(theme);
                    } else {
                        return Default::default();
                    }
                }
                let renderlist = cross_vec_str(&themelist, &widgitlist);
                Self {
                    themelist: themelist,
                    widgitlist: widgitlist,
                    renderlist: renderlist,
                }
            } else {
                Default::default()
            }
        } else {
            Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct CssRenderListRef<'a> {
    themelist: Vec<&'a str>,
    Renderlist: HashMap<&'a str, HashMap<&'a str, &'a str>>,
}
