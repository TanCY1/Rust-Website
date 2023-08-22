use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pages::home::Home;
use crate::components::pages::NotFound::NotFound;


#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html{
    match route {
        Route::Home => html!(<Home />),
        Route::NotFound => html!(<NotFound />)
    }
}