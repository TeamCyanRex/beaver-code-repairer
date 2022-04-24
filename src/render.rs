mod rander {
    #[macro_use]
    extern crate lazy_static;

    use std::collections::HashMap;
    
    lazy_static! {
        static ref RENDERNOW: HashMap<&'static str, &'static str> = {
            let mut m = HashMap::new();
            m.insert("color", "dark-color");
            m
        };
        static ref CLASS: &'static str = "dark";
        static ref COUNT: usize = RENDERNOW.len();
    }
    fn change_theme(theme:&str) {
        for (key, val) in RENDERNOW.iter(){
            RENDERNOW[key]=RENDERLIST[theme][key];
        }
    }
    lazy_static! {
        static ref RENDERLIST:HashMap<&'static str,HashMap<&'static str, &'static str>> =
    }
}
