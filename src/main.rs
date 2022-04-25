#[macro_use]
extern crate lazy_static;
use yew::prelude::*;
mod corefunc;
mod render;
use render::themes::get_render;
#[function_component(App)]
fn hello_world() -> Html {
    let color = get_render("color");
    html! {
        <p class={classes!(&color)}>{"Hello"}</p>
    }
}
fn main() {
    yew::start_app::<App>();
}
