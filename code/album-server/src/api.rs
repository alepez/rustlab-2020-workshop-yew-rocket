use super::Database;
use album_db::{Credentials, Image, ImageId, Images};
use core::convert::TryFrom;
use rocket::http::{Cookie, Cookies, SameSite};
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::{delete, get, post, put, routes, Route, State};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

pub fn routes() -> Vec<Route> {
    routes![index, login, images, image_preview, image_delete, image_put]
}

#[get("/")]
fn index() -> Json<String> {
    Json("hello".to_string())
}

#[post("/login", data = "<credentials>")]
fn login(
    mut cookies: Cookies,
    credentials: Json<Credentials>,
) -> Result<Json<AuthorizedUser>, rocket::http::Status> {
    let username = credentials.0.username;
    let password = credentials.0.password;

    if username == "admin" && password == "pass" {
        let auth = AuthorizedUser { username };
        cookies.add_private(auth.clone().into());
        Ok(Json(auth))
    } else {
        Err(rocket::http::Status::Unauthorized)
    }
}

#[get("/images")]
fn images(_u: AuthorizedUser, db: State<Database>) -> Json<Images> {
    let db = db.0.read().unwrap();
    Json(db.list_images().clone())
}

#[get("/images/<image>/preview.jpg")]
fn image_preview(_u: AuthorizedUser, db: State<Database>, image: ImageId) -> Option<Vec<u8>> {
    let db = db.0.read().ok()?;
    let path = image.preview_path(&db);
    std::fs::read(path).ok()
}

#[delete("/images/<image>")]
fn image_delete(_u: AuthorizedUser, db: State<Database>, image: ImageId) -> Json<Images> {
    let mut db = db.0.write().unwrap();
    db.delete_image(image);
    Json(db.list_images().clone())
}

#[put("/images/<_image>", data = "<image>")]
fn image_put(
    _u: AuthorizedUser,
    db: State<Database>,
    _image: ImageId,
    image: Json<Image>,
) -> Json<Images> {
    let mut db = db.0.write().unwrap();
    db.update_image(image.0);
    Json(db.list_images().clone())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthorizedUser {
    username: String,
}

const AUTH_COOKIE_KEY: &str = "auth";

impl TryFrom<rocket::http::Cookie<'_>> for AuthorizedUser {
    type Error = ();
    fn try_from(cookie: rocket::http::Cookie<'_>) -> Result<Self, Self::Error> {
        let json = cookie.value();
        serde_json::from_str(json).or(Err(()))
    }
}

impl<'a> Into<rocket::http::Cookie<'a>> for AuthorizedUser {
    fn into(self) -> rocket::http::Cookie<'a> {
        let json = serde_json::to_string(&self).unwrap();
        Cookie::build(AUTH_COOKIE_KEY, json)
            .http_only(true)
            .same_site(SameSite::Lax)
            .finish()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthorizedUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<AuthorizedUser, ()> {
        request
            .cookies()
            .get_private(AUTH_COOKIE_KEY)
            .and_then(|cookie| AuthorizedUser::try_from(cookie).ok())
            .into_outcome((rocket::http::Status::Unauthorized, ()))
    }
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
