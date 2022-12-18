use std::{
    fs, io,
    path::{self, Path},
};

use handlebars::Handlebars;
use walkdir::WalkDir;

use crate::compilers::{DefaultCompiler, FileCompiler, MarkdownCompiler};

const BUILD_DIR: &str = "public";
const PAGES_DIR: &str = "pages";
const TEMPLATES_DIR: &str = "templates";
const DATA_DIR: &str = "data";

pub fn build_pages(path: &str, registry: &Handlebars) -> io::Result<()> {
    let pages = Path::join(Path::new(path), Path::new(PAGES_DIR));
    let public = Path::new(BUILD_DIR);

    // iterator of paths of all files in the directory
    let paths = WalkDir::new(&pages)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| f.file_type().is_file());

    // fs operations
    let _ = fs::remove_dir_all(BUILD_DIR);

    for entry in paths {
        let path = entry.path();

        let mut out = Path::join(public, path.strip_prefix(&pages).unwrap());
        println!("{:?}", out);

        match path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase()
            .as_str()
        {
            // use polymorphism later
            "md" | "markdown" => MarkdownCompiler.compile(path, &mut out, registry)?,
            _ => {
                DefaultCompiler.compile(path, &mut out, registry)?;
            }
        }
    }

    Ok(())
}

pub fn gen_templates(path: &str, registry: &mut Handlebars) {
    let templates = Path::new(TEMPLATES_DIR);
    WalkDir::new(Path::new(path).join(templates))
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| f.file_type().is_file())
        .filter(|f| {
            f.path()
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase()
                .as_str()
                == "html"
        })
        .for_each(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            let name = &name[0..name.len() - 5];
            registry.register_template_file(name, entry.path()).unwrap()
        });
}

pub fn gen_static() {}

pub fn register_data(path: &str) {
    let data = Path::new(DATA_DIR);
    WalkDir::new(Path::new(path).join(data));
}
