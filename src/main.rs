use yew::prelude::*;
mod corefunc;

#[function_component(App)]
fn hello_world() -> Html {
    html! {
        <p class={classes!("cyan")}>{"Hello"}</p>
    }
}
fn main() {
    yew::start_app::<App>();
}
