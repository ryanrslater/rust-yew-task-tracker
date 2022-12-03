use yew::prelude::*;

mod style;

use crate::style::Style;

#[function_component(App)]
fn app() -> Html {
    html! {

        <html>
        <head>
        <Style />
        </head>
        <body>
        <main>
        <form>
        <input />
        <button>{"SUBMIT"}</button>
        </form>
        </main>
        </body>
        </html>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
