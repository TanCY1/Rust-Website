use yew::prelude::*;
use stylist::{yew::styled_component, Style};
const STYLE_FILE: &str = include_str!("../../style.css");


#[function_component(Home)]
pub fn home() -> Html {
    let benefits_of_rust = vec![
        "Memory safe",
        "Fast and High Performace",
        "Expansive ecosystem",
        "Easy to learn",
    ];
    let styles = Style::new(STYLE_FILE).unwrap();
    html! {
        <div class={styles}>
        
        <div class="yellowbackg">
            <h1 style="text-align:center;">{ "The Rust Programming Language" }</h1>
            <img src="https://trunkrs.dev/rustacean-flat-happy.svg" style="display:block; width:50%; margin-left:auto; margin-right:auto;"/>
        </div>
        <h1>{"Benefits of Rust"}</h1>
        <div class="whitebackg">
            <ul>{string_vector_to_html(benefits_of_rust)}</ul>
        </div>
    </div>
    }
}

fn string_vector_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter()
        .map(|item| html! {<li><h2>{item}</h2></li>})
        .collect()
}
