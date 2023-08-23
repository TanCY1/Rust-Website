mod components;
mod router;
use crate::router::{switch, Route};
use stylist::{yew::styled_component};
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::Switch;



#[styled_component(App)]
fn app() -> Html {

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

