use rocket::{get, routes, Route};
use rocket_contrib::json::Json;

pub fn routes() -> Vec<Route> {
    routes![index]
}

#[get("/")]
fn index() -> Json<String> {
    Json("hello".to_string())
}
