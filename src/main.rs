#[macro_use]
extern crate lazy_static;
use yew::prelude::*;
mod corefunc;
mod render;
use render::themes::get_render;
mod widgets;
use widgets::buttons::ButtonT;
#[function_component(App)]
fn hello_world() -> Html {
    html! {
        <>
        <ButtonT val=1></ButtonT>
        <p>{get_render("color")}</p>
        </>
    }
}

fn main() {
   yew::start_app::<App>();
   //println!("{}",THEME_CENTERc.get_render("color"))
}
