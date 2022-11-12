mod ext;

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use ext::FileCompiler;

use rocket::fs::{relative, NamedFile};
use walkdir::WalkDir;

use crate::ext::MarkdownCompiler;

#[macro_use]
extern crate rocket;

fn build(path: &str) -> io::Result<()> {
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

#[launch]
fn rocket() -> _ {
    // let compilers: Vec<&dyn FileCompiler> = vec![&MarkdownCompiler {}];
    // let config: Config = Config { compilers: vec![&MarkdownCompiler {}] };
    // let compilers: Vec<&dyn FileCompiler> = vec![&MarkdownCompiler {}, &YewCompiler {}];
    build("dir").unwrap();

    rocket::build().mount("/", routes![files])
    // .ignite().await?
    // .mount("/", routes![files])
    // .mount("/", FileServer::from(relative!("dir")))
}

struct Config<'a> {
    compilers: Vec<&'a dyn FileCompiler>,
}
