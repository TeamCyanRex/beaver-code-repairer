use crate::corefunc::css_resolver::CssRenderList;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    static ref DEFAULT_THEME: Mutex<String> = Mutex::new(String::from("river"));
    static ref CSS_RENDER_LIST: CssRenderList = CssRenderList::new(Path::new(r"css"));
    static ref RENDERLIST: HashMap<String, HashMap<String, String>> =
        CSS_RENDER_LIST.renderlist.clone();
    static ref THEMES: Vec<String> = CSS_RENDER_LIST.themelist.clone();
    static ref RENDERNOW: HashMap<String, Mutex<String>> = {
        let mut now = HashMap::new();
        let first_render: &HashMap<String, String> = &RENDERLIST[&get_default_theme()];
        for (k, v) in first_render.iter() {
            now.insert(k.to_string(), Mutex::new(v.to_string()));
        }
        now
    };
    static ref THEME_COUNT: usize = THEMES.len();
}
pub fn get_default_theme() -> String {
    match DEFAULT_THEME.lock().map(|mt| mt.to_string()) {
        Ok(res) => res,
        _ => panic!("from get_default_theme:get default theme name from poisoned mutex"),
    }
}
pub fn set_default_theme(theme: &str) {
    DEFAULT_THEME.lock().map(|mut mt| {
        mt.clear();
        mt.push_str(theme);
    });
}
pub fn change_theme(theme: &str) {
    for (key, val) in RENDERNOW.iter() {
        RENDERNOW[key].lock().map(|mut widget| {
            widget.clear();
            widget.push_str(&RENDERLIST[theme][key])
        });
    }
}
pub fn get_render(widget: &str) -> String {
    match RENDERNOW[widget]
        .lock()
        .map(|mutex_res| mutex_res.to_string())
    {
        Ok(res) => res,
        _ => panic!("from get_render:get themed widget name from poisoned mutex"),
    }
}
