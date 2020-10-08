use super::Database;
use album_db::{Image, Images};
use rocket::{delete, get, routes, Route, State};
use rocket_contrib::json::Json;

pub fn routes() -> Vec<Route> {
    routes![index, images, image_preview, image_delete]
}

#[get("/")]
fn index() -> Json<String> {
    Json("hello".to_string())
}

#[get("/images")]
fn images(db: State<Database>) -> Json<Images> {
    let db = db.0.read().unwrap();
    Json(db.list_images().clone())
}

#[get("/images/<image>/preview.jpg")]
fn image_preview(db: State<Database>, image: Image) -> Option<Vec<u8>> {
    let db = db.0.read().unwrap();
    let path = image.preview_path(&db);
    std::fs::read(path).ok()
}

#[delete("/images/<image>")]
fn image_delete(db: State<Database>, image: Image) -> Result<(), ()> {
    let mut db = db.0.write().unwrap();
    db.delete_image(&image);
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
