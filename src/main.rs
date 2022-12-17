mod build;
mod compilers;

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use handlebars::Handlebars;
use rocket::{
    fs::{relative, NamedFile},
    State,
};

#[macro_use]
extern crate rocket;

/**
site structure:

targetDir
├── pages
└── templates
*/

#[get("/<path..>")]
async fn files(path: PathBuf) -> Option<NamedFile> {
    let mut path = Path::new(relative!("public")).join(path);
    if path.is_dir() {
        path.push("index.html");
    }
    NamedFile::open(path).await.ok()
}

#[get("/<path..>")]
async fn registry_files(path: PathBuf, registry: &State<Handlebars<'_>>) -> String {
    let mut path = Path::new(relative!("public")).join(path);
    if path.is_dir() {
        path.push("index.html");
    }
    registry.render(path.to_str().unwrap(), &()).unwrap()
}

#[launch]
fn rocket() -> _ {
    let mut registry = Handlebars::new();
    let mut templates = Handlebars::new();

    // let config: Config = Config { compilers: vec![&MarkdownCompiler {}] };
    build::gen_templates("dir/", &mut templates);
    build::build("dir/", &templates).unwrap();

    rocket::build()
        .manage(registry)
        // .mount("/", routes![registry_files])
        // .ignite().await?
        .mount("/", routes![files])
    // .mount("/", FileServer::from(relative!("dir")))
}
