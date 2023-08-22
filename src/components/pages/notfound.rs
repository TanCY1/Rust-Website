use yew::prelude::*;

#[function_component(NotFound)]
pub fn notfound() -> Html {
    html! {
      <h1>{"404 Not Found"}</h1>
    }
}