use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let todos = use_state(|| vec!["Learn Rust".to_string(), "Build a web with Yew".to_string()]);
    html! {
        <div>
            <h1>{ "Task list" }</h1>
            <ul>
                { for todos.iter().map(|item| html!{ <li>{ item }</li> }) }
            </ul>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
