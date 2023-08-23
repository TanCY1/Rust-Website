use yew::prelude::*;
use stylist::{Style};
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
        <div class="list">
            <ul>{string_vector_to_html(benefits_of_rust)}</ul>
            <img style="width: 25vh; height: 25vh;
          " src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1024px-Rust_programming_language_black_logo.svg.png"/>
        </div>
        <div class="iframe">
            <iframe width="560" height="315" src="https://www.youtube.com/embed/5C_HPTJg5ek" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen=true></iframe>
        </div>
        </div>
    </div>
    }
}

fn string_vector_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter()
        .map(|item| html! {<li><h2>{item}</h2></li>})
        .collect()
}
