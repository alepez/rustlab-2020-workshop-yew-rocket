use album_db::{list_images, Image, Images};
use rocket::{delete, get, routes, Route};
use rocket_contrib::json::Json;

pub fn routes() -> Vec<Route> {
    routes![index, images, image_preview, image_delete]
}

#[get("/")]
fn index() -> Json<String> {
    Json("hello".to_string())
}

#[get("/images")]
fn images() -> Option<Json<Images>> {
    list_images().map(Json).ok()
}

#[get("/images/<image>/preview.jpg")]
fn image_preview(image: Image) -> Option<Vec<u8>> {
    let path = image.preview_path();
    std::fs::read(path).ok()
}

#[delete("/images/<image>")]
fn image_delete(image: Image) -> Result<(), ()> {
    log::info!("Delete image {:?}", image);
    Ok(())
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
