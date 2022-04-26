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
fn theme_center_test() {
    let mut themes_center=ThemesCenter::new(Path::new(r"css"), "river");
    assert_eq!(themes_center.get_default_theme(),"river");
    themes_center.set_default_theme("lab");
    assert_eq!(themes_center.get_default_theme(),"lab");
    assert_eq!(themes_center.get_render("color"),"river-color");
    themes_center.change_theme("lab");
    assert_eq!(themes_center.get_render("color"),"lab-color");
}
#[test]
fn themes_center_default_test() {
    let mut themes_center=ThemesCenter::default_init();
    println!("{:#?}",themes_center);
    assert_eq!(themes_center.get_default_theme(),"dark");
    themes_center.set_default_theme("lab");
    assert_eq!(themes_center.get_default_theme(),"lab");
    assert_eq!(themes_center.get_render("color"),"dark-color");
    themes_center.change_theme("lab");
    assert_eq!(themes_center.get_render("color"),"lab-color");
    themes_center.change_theme("light");
    assert_eq!(themes_center.get_render("color"),"light-color");
}
#[test]
fn themes_center_mutex_default_test() {
    let mut themes_center=ThemesCenterWithMutexDefaultTheme::default_init();
    println!("{:#?}",themes_center);
    assert_eq!(themes_center.get_default_theme(),"dark");
    themes_center.set_default_theme("lab");
    assert_eq!(themes_center.get_default_theme(),"lab");
    assert_eq!(themes_center.get_render("color"),"dark-color");
    themes_center.change_theme("lab");
    assert_eq!(themes_center.get_render("color"),"lab-color");
    themes_center.change_theme("light");
    assert_eq!(themes_center.get_render("color"),"light-color");
}