use yew::prelude::*;
pub mod sidebar;

use crate::sidebar::Sidebar;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div style={format!("display: flex; box-styling: border-box")}>
        <Sidebar />
        <div></div>
        <div></div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
