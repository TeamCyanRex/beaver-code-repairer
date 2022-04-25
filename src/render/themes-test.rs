use crate::corefunc::css_resolver::CssRenderList;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use super::themes::*;
#[test]
fn render_now_test() {
    let renderlist = CssRenderList::new(Path::new(r"css"));
    let render_now: HashMap<String, Mutex<String>> = {
        let mut now = HashMap::new();
        let first_render: &HashMap<String, String> = match renderlist.renderlist.iter().next() {
            Some((k, v)) => v,
            _ => panic!(),
        };
        for (k, v) in first_render.iter() {
            now.insert(k.to_string(), Mutex::new(v.to_string()));
        }
        now
    };
    println!("{:#?}", render_now);
    // assert_eq!(render_now,)
}

#[test]
fn change_theme_test() {
   assert_eq!(get_render("color"),"river-color");
  // assert_eq!(get_default_theme(),"dark");
   change_theme("lab");
   assert_eq!(get_render("color"),"lab-color");
   set_default_theme("lab");
   assert_eq!(get_default_theme(),"lab");
}
