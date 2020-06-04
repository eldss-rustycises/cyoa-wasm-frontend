use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct StoryArc {
    pub title: String,
    pub story: Vec<String>,
    pub options: Vec<Opt>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Opt {
    pub text: String,
    pub arc: String,
}
