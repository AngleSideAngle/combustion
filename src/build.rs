use std::{fs, io, path::{Path, self}};

use handlebars::Handlebars;
use walkdir::WalkDir;

use crate::ext::{FileCompiler, MarkdownCompiler, DefaultCompiler};

const BUILD_DIR: &str = "public";
const PAGES_DIR: &str = "pages";
const TEMPLATES_DIR: &str = "templates";

pub fn build(path: &str, registry: &mut Handlebars) -> io::Result<()> {
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
            .as_str() {
                // use polymorphism later
                "md" | "markdown" => MarkdownCompiler.compile(path, &mut out)?,
                _ => {
                    DefaultCompiler.compile(path, &mut out)?;
                }
            }
    }
    
    Ok(())
}

pub fn gen_templates(path: &str) {
    let mut templates = Handlebars::new();
    let templates = Path::new(TEMPLATES_DIR);
    for entry in WalkDir::new(Path::new(path).join(templates))
        .into_iter()
        .skip(1)
        .filter_map(|f| f.ok())
    {
        println!("{}", entry.file_name().to_str().get_or_insert("NA"));
    }
}
