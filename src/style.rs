use yew::prelude::*;



#[function_component(Style)]
pub fn style() -> Html {
    html! {
        <style>
        {"
            body {
                margin: 0;
                background-color: #f2f2f2;
            }

            form {
                width: 500px;
                margin: 20px auto;
                background-color: white;
                padding: 30px;
                border-radius: 4px;
            }
            input {
                width: 100%;
            }
        "}
        </style>
    }
}