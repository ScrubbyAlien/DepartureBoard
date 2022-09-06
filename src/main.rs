#[macro_use]
extern crate rocket;
extern crate lazy_static;

pub mod lib {
    pub mod config;
    pub mod util;
}

use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

use lib::util::get_res_robot_response;

#[get("/api")]
async fn api() -> String {
    get_res_robot_response().await.unwrap()
}

#[get("/<file..>")]
async fn index(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/static/").join(file))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![api, index])
}
