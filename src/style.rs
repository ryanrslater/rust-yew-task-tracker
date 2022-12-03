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
            .button-wrapper {
                display: flex;
                justify-content: flex-end;
                margin-top: 20px
            }
            button {
                display:inline-block;
                padding:0.3em 1.2em;
                margin:0 0.1em 0.1em 0;
                border:0.16em solid rgba(255,255,255,0);
                border-radius:2em;
                box-sizing: border-box;
                text-decoration:none;
                font-family:'Roboto',sans-serif;
                font-weight:300;
                color:#FFFFFF;
                text-shadow: 0 0.04em 0.04em rgba(0,0,0,0.35);
                text-align:center;
                transition: all 0.2s;
                background-color:#9a4ef1;
            }
            button:hover {
                border-color: rgba(255,255,255,1);
                cursor: pointer;
            }
            @media all and (max-width:30em){
                button{
                    display:block;
                    margin:0.2em auto;
                    }
                }
        "}
        </style>
    }
}
