#![feature(decl_macro)]

mod api;

use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;

fn static_files(rocket: Rocket) -> Result<Rocket, Rocket> {
    const DEFAULT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static");

    let dir = rocket
        .config()
        .get_str("static_dir")
        .unwrap_or(DEFAULT_DIR)
        .to_string();

    let static_files = StaticFiles::from(dir.clone());

    let rocket = rocket.mount("/", static_files);

    Ok(rocket)
}

fn ignite() -> rocket::Rocket {
    dotenv::dotenv().ok();

    rocket::ignite()
        .attach(AdHoc::on_attach("Static files", static_files))
        .mount("/api", api::routes())
}

fn main() {
    env_logger::init();
    ignite().launch();
}
