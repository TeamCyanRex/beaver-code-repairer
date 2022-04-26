use crate::corefunc::css_resolver::CssRenderList;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
pub trait TrThemesCenter {
    fn new<P: AsRef<Path>>(css_folder: P, default_theme_name: &str) -> Self;
    fn default_init() -> Self;
    fn get_default_theme(&self) -> String;
    fn set_default_theme(&self, theme: &str);
    fn change_theme(&self, theme: &str);
    fn get_render(&self, widget: &str) -> String;
}
#[derive(Debug)]
pub struct ThemesCenter {
    default_theme: String,
    css_render_list: CssRenderList,
    render_list: HashMap<String, HashMap<String, String>>,
    theme: Vec<String>,
    render_now: HashMap<String, Mutex<String>>,
    theme_count: usize,
}
impl TrThemesCenter for ThemesCenter {
    fn new<P: AsRef<Path>>(css_folder: P, default_theme_name: &str) -> ThemesCenter {
        let default_theme = String::from(default_theme_name);
        let css_render_list = CssRenderList::new(css_folder);
        let render_list = css_render_list.renderlist.clone();
        let theme = css_render_list.themelist.clone();
        let render_now = {
            let mut now = HashMap::new();
            let first_render: &HashMap<String, String> = &render_list[&default_theme];
            for (k, v) in first_render.iter() {
                now.insert(k.to_string(), Mutex::new(v.to_string()));
            }
            now
        };
        let theme_count = theme.len();
        Self {
            default_theme: default_theme,
            css_render_list: css_render_list,
            render_list: render_list,
            theme: theme,
            render_now: render_now,
            theme_count: theme_count,
        }
    }
    fn default_init() -> Self {
        Self::new(r"dist", "dark")
    }
    fn get_default_theme(&self) -> String {
        self.default_theme.to_string()
    }
    fn set_default_theme(&self, theme: &str) {
        unimplemented!()
    }
    fn change_theme(&self, theme: &str) {
        for (key, val) in self.render_now.iter() {
            self.render_now[key].lock().map(|mut widget| {
                widget.clear();
                widget.push_str(&self.render_list[theme][key])
            });
        }
    }
    fn get_render(&self, widget: &str) -> String {
        match self.render_now[widget]
            .lock()
            .map(|mutex_res| mutex_res.to_string())
        {
            Ok(res) => res,
            _ => panic!("from get_render:get themed widget name from poisoned mutex"),
        }
    }
}
#[derive(Debug)]
pub struct ThemesCenterWithMutexDefaultTheme {
    default_theme: Mutex<String>,
    css_render_list: CssRenderList,
    render_list: HashMap<String, HashMap<String, String>>,
    theme: Vec<String>,
    render_now: HashMap<String, Mutex<String>>,
    theme_count: usize,
}
impl TrThemesCenter for ThemesCenterWithMutexDefaultTheme {
    fn new<P: AsRef<Path>>(
        css_folder: P,
        default_theme_name: &str,
    ) -> ThemesCenterWithMutexDefaultTheme {
        let default_theme = String::from(default_theme_name);
        let css_render_list = CssRenderList::new(css_folder);
        let render_list = css_render_list.renderlist.clone();
        let theme = css_render_list.themelist.clone();
        let render_now = {
            let mut now = HashMap::new();
            let first_render: &HashMap<String, String> = &render_list[&default_theme];
            for (k, v) in first_render.iter() {
                now.insert(k.to_string(), Mutex::new(v.to_string()));
            }
            now
        };
        let theme_count = theme.len();
        Self {
            default_theme: Mutex::new(default_theme),
            css_render_list: css_render_list,
            render_list: render_list,
            theme: theme,
            render_now: render_now,
            theme_count: theme_count,
        }
    }
    fn default_init() -> Self {
        Self::new("dist", "dark")
    }
    fn get_default_theme(&self) -> String {
        match self.default_theme.lock().map(|mt| mt.to_string()) {
            Ok(res) => res,
            _ => panic!("from get_default_theme:get default theme name from poisoned mutex"),
        }
    }
    fn set_default_theme(&self, theme: &str) {
        self.default_theme.lock().map(|mut mt| {
            mt.clear();
            mt.push_str(theme);
        });
    }
    fn change_theme(&self, theme: &str) {
        for (key, val) in self.render_now.iter() {
            self.render_now[key].lock().map(|mut widget| {
                widget.clear();
                widget.push_str(&self.render_list[theme][key])
            });
        }
    }
    fn get_render(&self, widget: &str) -> String {
        match self.render_now[widget]
            .lock()
            .map(|mutex_res| mutex_res.to_string())
        {
            Ok(res) => res,
            _ => panic!("from get_render:get themed widget name from poisoned mutex"),
        }
    }
}

//lazy_static! can not work with yew now!

lazy_static! {
    static ref THEME_CENTER: ThemesCenterWithMutexDefaultTheme =
        ThemesCenterWithMutexDefaultTheme::new(Path::new(r"css"), "river");
}
pub fn get_render(widget: &str) -> String {
    THEME_CENTER.get_render(widget)
}
pub fn change_theme(theme: &str) {
    THEME_CENTER.change_theme(theme)
}
pub fn get_default_theme() -> String {
    THEME_CENTER.get_default_theme()
}
pub fn set_default_theme(theme: &str) {
    THEME_CENTER.set_default_theme(theme)
}
