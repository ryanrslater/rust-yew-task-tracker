use yew::prelude::*;
pub mod center;
pub mod sidebar;

use crate::center::Center;
use crate::sidebar::Sidebar;

#[function_component(App)]
fn app() -> Html {
    html! {


        <div style={format!("display: flex")}>
        <style>
        {"
        body {
            margin: 0;
        }
"}
        </style>
        <Sidebar />
       <Center />
        <div style={format!("width: 500px; height: 100vh;")}>
        <h2>{"Current Order"}</h2>
        </div>
        </div>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
