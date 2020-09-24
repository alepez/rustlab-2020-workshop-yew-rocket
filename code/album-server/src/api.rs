use rocket::{get, routes, Route};
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Image {
    filename: String,
}

#[derive(Debug, Default, Serialize)]
struct Images(Vec<Image>);

pub fn routes() -> Vec<Route> {
    routes![index, images]
}

#[get("/")]
fn index() -> Json<String> {
    Json("hello".to_string())
}

#[get("/images")]
fn images() -> Json<Images> {
    Json(Images::default())
}
