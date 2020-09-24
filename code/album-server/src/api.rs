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
    Json(fake_images())
}

fn fake_images() -> Images {
    Images(vec![Image {
        filename: "dog.3145.jpg".into(),
    }])
}

#[cfg(test)]
mod test {
    use super::super::ignite;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn api_images_get_ok() {
        let client = Client::new(ignite()).unwrap();
        let mut response = client.get("/api/images").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.body_string().unwrap(),
            "[{\"filename\":\"dog.3145.jpg\"}]"
        );
    }
}
