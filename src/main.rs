mod ext;

use std::{
    fs, io,
    path::{Path, PathBuf}, sync::Arc,
};

use ext::FileCompiler;

use handlebars::Handlebars;
use rocket::{fs::{relative, NamedFile}, tokio::sync::{Mutex, RwLock}, State};
use walkdir::WalkDir;

use crate::ext::MarkdownCompiler;

#[macro_use]
extern crate rocket;

fn build(path: &str, registry: &Handlebars) -> io::Result<()> {
    if let Err(e) = fs::create_dir("public") {
        fs::remove_dir_all("public").unwrap();
        fs::create_dir("public").unwrap();
    }
    let public = Path::new("public");

    for entry in WalkDir::new(path) {
        if let Ok(e) = entry {
            let mut new_path = Path::join(public, e.path());
            if e.file_type().is_dir() {
                fs::create_dir(new_path).unwrap();
            } else {
                match e
                    .path()
                    .extension()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase()
                    .as_str()
                {
                    "md" | "markdown" => MarkdownCompiler.compile(e.path(), &mut new_path)?,
                    _ => {
                        fs::copy(e.path(), new_path);
                    }
                }
            }
        }
    }
    Ok(())
}

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

    // let config: Config = Config { compilers: vec![&MarkdownCompiler {}] };
    build("dir", &registry).unwrap();

    rocket::build()
        .manage(registry)
        .mount("/", routes![registry_files])
    // .ignite().await?
    // .mount("/", routes![files])
    // .mount("/", FileServer::from(relative!("dir")))
}

struct Config<'a> {
    compilers: Vec<&'a dyn FileCompiler>,
}
