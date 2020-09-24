use album_db::{list_images, Image, Images};
use rocket::{get, routes, Route};
use rocket_contrib::json::Json;

pub fn routes() -> Vec<Route> {
    routes![index, images, image_preview]
}

#[get("/")]
fn index() -> Json<String> {
    Json("hello".to_string())
}

#[get("/images")]
fn images() -> std::io::Result<Json<Images>> {
    list_images().map(Json)
}

#[get("/images/<id>/preview.jpg")]
fn image_preview(id: usize) -> std::io::Result<Vec<u8>> {
    let image = Image::from_id(id);
    let path = image.preview_path();
    std::fs::read(path)
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
        assert!(response.body_string().unwrap().len() > 0);
    }
}
