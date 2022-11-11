use std::{
    fs::{self, read, File, FileType},
    io::{self, BufReader, Error, Write},
    path::{Path, PathBuf},
    string,
};

use pulldown_cmark::{html, Options, Parser};
use rocket::{
    fs::{relative, FileServer, NamedFile},
    tokio::fs::create_dir,
    Build,
};
use walkdir::WalkDir;

#[macro_use]
extern crate rocket;

async fn build(path: &str) -> io::Result<()> {
    if let Err(e) = fs::create_dir("public") {
        fs::remove_dir_all("public").unwrap();
        fs::create_dir("public").unwrap();
    }
    let public = Path::new("public");

    for entry in WalkDir::new(path) {
        if let Ok(e) = entry {
            let mut new_path = Path::join(public, e.path());
            // jank file type checking because it doesn't support pattern matching
            if e.file_name().to_string_lossy().ends_with(".md") {
                println!("{}", e.path().to_string_lossy());
                let file = File::open(e.path())?;
                println!("file opened");
                let text = String::from_utf8_lossy(&read(e.path())?).into_owned();
                println!("text: {}", text);
                // let mut reader = BufReader::new(file);
                let parser = Parser::new(&text);
                let mut output = String::new();
                html::push_html(&mut output, parser);
                println!("output: {}", output);
                println!("new path: {}", Path::join(public, e.path()).to_string_lossy());
                new_path.set_extension("html");
                let mut out = File::create(new_path)?;
                println!("made output file");
                out.write(output.as_bytes())?;
                println!("{}", output);
            } else if e.file_type().is_file() {
                fs::copy(e.path(), new_path).unwrap();
            } else if e.file_type().is_dir() {
                fs::create_dir(new_path).unwrap();
            }

        }
    }
    println!("finished");
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

#[rocket::main]
async fn main() {
    build("dir").await.unwrap();

    let res = rocket::build().mount("/", routes![files]).launch().await;
    // .ignite().await?
    // .mount("/", routes![files])
    // .mount("/", FileServer::from(relative!("dir")))
}
