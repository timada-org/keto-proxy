use axum::{response::IntoResponse, routing::get, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Rule {
    subject_id: String,
    relation: String,
    namespace: String,
    object: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthorizeResponse {
    allowed: bool,
}

async fn authorize() -> impl IntoResponse {
    let rule = Rule {
        subject_id: "".to_owned(),
        relation: "".to_owned(),
        namespace: "".to_owned(),
        object: "".to_owned(),
    };
    let result: AuthorizeResponse = reqwest::Client::new()
        .post("https://keto.timada.localhost/relation-tuples/check")
        .json(&rule)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("{:?}", result);

    "yolo"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/authorize", get(authorize));

    axum::Server::bind(&([127, 0, 0, 1], 3001).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
