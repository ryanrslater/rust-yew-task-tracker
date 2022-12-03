use yew::prelude::*;

mod style;

use crate::style::Style;

struct Task {
    body: String
}

#[function_component(App)]
fn app() -> Html {
    let task_list: UseStateHandle<Vec<Task>> = use_state(|| []);
    let title_state = use_state(|| "");

    let on_capture_title = {
        let title_state = title_state.clone();
            Callback::from(move |e: Event| {
                title_state.set(e.target(AttrValue))
            })
    };
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
