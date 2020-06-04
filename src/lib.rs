// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{browser::fetch, prelude::*, *};

pub mod types;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
type Model = types::StoryArc;

impl Default for Model {
    fn default() -> Self {
        Self {
            title: "".to_owned(),
            story: vec![],
            options: vec![],
        }
    }
}

// ------ ------
//    Update
// ------ ------

async fn fetch_story_arc(arc: String) -> Result<Model, fetch::FetchError> {
    let url = format!("http://localhost:8085/arc/{}", arc);
    // If status is good, returns the result of parsing the json body into Model
    fetch::fetch(url).await?.check_status()?.json().await
}

// `Msg` describes the different events you can modify state with.
enum Msg {
    FetchArc(String),
    FetchArcComplete(Model),
    Error,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchArc(arc) => {
            orders.perform_cmd(async {
                match fetch_story_arc(arc).await {
                    Ok(data) => Msg::FetchArcComplete(data),
                    Err(err) => {
                        error!("problem fetching data: {:?}", err);
                        orders.skip();
                        Msg::Error
                    }
                }
            });
        }
        Msg::FetchArcComplete(mdl) => {
            model.title = mdl.title;
            model.story = mdl.story;
            model.options = mdl.options;
        }
        Msg::Error => {}
    }
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![header(),]
}

fn header() -> Node<Msg> {
    let base_style = style! {
        St::Position => "fixed",
        St::Display => "flex",
        St::JustifyContent => "center",
        St::Left => 0,
        St::Top => 0,
        St::Width => "100%",
        St::Height => "56px",
        St::Padding => 0,
        St::Background => "#007d9c",
        St::BoxShadow => "0 0 5px rgba(0, 0, 0, 0.5)",
        St::ZIndex => 50,
    };

    let heading_link_style = style! {
        St::Margin => 0,
        St::Padding => "0 15px",
        St::FontSize => "24px",
        St::LineHeight => "56px",
        St::FontWeight => 400,
        St::Color => "#FFF",
    };

    let link_style = style! {
        St::TextDecoration => "none",
    };

    header![
        &base_style,
        h1![
            &heading_link_style,
            a![
                &link_style,
                attrs! {At::Href => "https://courses.calhoun.io/", At::Target => "_"},
                "Gophercises - Choose Your Own Adventure (Except Rust)"
            ],
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
