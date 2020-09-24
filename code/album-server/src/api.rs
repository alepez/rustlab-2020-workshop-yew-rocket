use rocket::{get, routes, Route};
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Image {
    id: usize,
    filename: String,
}

impl Image {
    fn from_id(id: usize) -> Self {
        let filename = format!("dog.{}.jpg", id);
        Self { id, filename }
    }

    fn preview_path(&self) -> std::path::PathBuf {
        let mut path =
            std::path::PathBuf::from("/home/pez/workspace/rustlab/rocket-yew-workshop/dogs");
        path.push(&self.filename);
        path
    }
}

#[derive(Debug, Default, Serialize)]
struct Images(Vec<Image>);

pub fn routes() -> Vec<Route> {
    routes![index, images, image_preview]
}

#[get("/")]
fn index() -> Json<String> {
    Json("hello".to_string())
}

#[get("/images")]
fn images() -> Json<Images> {
    Json(fake_images())
}

#[get("/images/<id>/preview.jpg")]
fn image_preview(id: usize) -> std::io::Result<Vec<u8>> {
    let image = Image::from_id(id);
    let path = image.preview_path();
    std::fs::read(path)
}

fn fake_images() -> Images {
    Images(vec![Image::from_id(3145)])
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
            "[{\"id\":3145,\"filename\":\"dog.3145.jpg\"}]"
        );
    }
}
