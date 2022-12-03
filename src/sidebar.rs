use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {

        <div style={format!("height: 100vh; width: 100px; display: flex; flex-direction: column; align-items: center; justify-content: space-between")}>
        <div><h1>{ "l" }</h1></div>
        <div style={format!("display: flex; flex-direction: column; width: 100%; align-items: center; justify-content: center")}>
            <hr style={format!("width: 80%")} />
            <h1>{ "l" }</h1>
            <h1>{ "l" }</h1>
        </div>
        </div>
    }
}
