use core::future::Future;
use rocket::serde::Deserialize;
use std::path::{Path, PathBuf};

use rocket::fairing::AdHoc;
use rocket::fs::NamedFile;

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
struct AppConfig {}

const DIST: &str = "dist/";

fn ui_open<P: AsRef<Path>>(file: P) -> impl Future<Output = Option<NamedFile>> {
    async move { NamedFile::open(Path::new(DIST).join(file)).await.ok() }
}
#[catch(404)]
async fn catch404() -> Option<NamedFile> {
    ui_open("page404.html").await
}

#[catch(500)]
async fn catch500() -> Option<NamedFile> {
    ui_open("page500.html").await
}

#[catch(401)]
async fn catch401() -> Option<NamedFile> {
    ui_open("page401.html").await
}

#[catch(403)]
async fn catch403() -> Option<NamedFile> {
    ui_open("page403.html").await
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    ui_open("index.html").await
}

#[get("/<asset..>")]
async fn assets(asset: PathBuf) -> Option<NamedFile> {
    ui_open(asset).await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![catch404, catch500, catch403, catch401])
        .mount("/", routes![assets, index])
        .attach(AdHoc::config::<AppConfig>())
}
