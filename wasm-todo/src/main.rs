use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let todos = use_state(|| vec!["Aprender Rust".to_string(), "Hacer una app Yew".to_string()]);
    html! {
        <div>
            <h1>{ "Lista de tareas" }</h1>
            <ul>
                { for todos.iter().map(|item| html!{ <li>{ item }</li> }) }
            </ul>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
