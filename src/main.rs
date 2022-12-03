use yew::prelude::*;

mod style;

use crate::style::Style;

struct Task {
    title: String,
    body: String
}

#[function_component(App)]
fn app() -> Html {
    let task_list = use_state(|| None);
    html! {

        <html>
        <head>
        <Style />
        </head>
        <body>
        <main>
        <form>
        <input />
        <div class="button-wrapper">
        <button type="submit">{"SUBMIT"}</button>
        </div>
        </form>
        </main>
        </body>
        </html>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
