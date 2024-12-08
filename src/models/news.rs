use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GNewsResponse {
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
}
