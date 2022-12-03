use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <div style={format!("background-color: red; height: 100vh; width: 100px")}>
        <h1>{ "Sidebar" }</h1>
        </div>
    }
}

