mod build;
mod compilers;

use std::{
    collections::BTreeMap,
    // fs, io,
    path::{Path, PathBuf},
};

use handlebars::Handlebars;
use rocket::{fs::relative, http::ContentType, tokio::fs, State};

use toml::Value;

#[macro_use]
extern crate rocket;

/**
site structure:

targetDir
├── pages
└── static (cloned, rather than compiled)
└── templates
└── data
*/

#[get("/<path..>")]
async fn ssr(
    path: PathBuf,
    templates: &State<Handlebars<'_>>,
    data: &State<BTreeMap<String, Value>>,
) -> (ContentType, String) {
    // does not use rocket templates to horrid lack of configurability

    // Template::render(path.file_name(), data);
    let mut path = Path::new(relative!("public")).join(path);
    if path.is_dir() {
        path.push("index.html");
    }

    // jank workaround to return variable content type
    // this is the only solution i found after several hours of looking
    (
        ContentType::from_extension(&path.extension().unwrap_or_default().to_string_lossy())
            .unwrap_or_default(),
        templates
            .render_template(&fs::read_to_string(&path).await.unwrap(), data.inner())
            .unwrap(),
    )
}

pub async fn start(path: &str) {
    let mut data: BTreeMap<String, Value> = BTreeMap::new();
    let mut templates = Handlebars::new();
    let mut registry = Handlebars::new();
    // registry.
    registry.set_strict_mode(true);

    // let config: Config = Config { compilers: vec![&MarkdownCompiler {}] };
    build::gen_templates(path, &mut templates);
    build::register_data(path, &mut data);
    build::build_pages(path, &templates).unwrap();

    let _ = rocket::build()
        .manage(templates)
        .manage(data)
        .mount("/", routes![ssr])
        .launch()
        .await
        .unwrap();
}
