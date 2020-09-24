use rocket::{get, routes, Route};
use rocket_contrib::json::Json;
use serde::Serialize;

const PREVIEWS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../dogs");

#[derive(Debug, Serialize)]
struct Image {
    id: usize,
}

impl Image {
    fn from_id(id: usize) -> Self {
        Self { id }
    }

    fn preview_path(&self) -> std::path::PathBuf {
        let filename = format!("dog.{}.jpg", self.id);
        let mut path = std::path::PathBuf::from(PREVIEWS_DIR);
        path.push(filename);
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
    Json(list_images().unwrap())
}

#[get("/images/<id>/preview.jpg")]
fn image_preview(id: usize) -> std::io::Result<Vec<u8>> {
    let image = Image::from_id(id);
    let path = image.preview_path();
    std::fs::read(path)
}

fn parse_id(filename: &str) -> Option<usize> {
    let name: String = filename.chars().skip(4).take_while(|&x| x != '.').collect();
    name.parse().ok()
}

fn list_images() -> Result<Images, std::io::Error> {
    let entries = std::fs::read_dir(PREVIEWS_DIR)?;

    let images = entries
        .filter_map(|res| res.ok())
        .filter_map(|res| {
            let name = res.file_name();
            parse_id(name.to_str()?)
        })
        .map(|id| Image::from_id(id))
        .collect();

    Ok(Images(images))
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
