mod build;
mod compilers;

use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};

use handlebars::{Handlebars, HelperDef};
use rocket::{http::ContentType, tokio::fs, State};

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
    vars: &State<Vars>,
) -> (ContentType, String) {
    // does not use rocket templates to horrid lack of configurability

    let mut path = vars.path.join(Path::new("public")).join(path);
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

pub struct Config {
    path: String,
    helpers: HashMap<String, Box<dyn HelperDef + Sync + Send>>,
}

impl Config {
    pub fn new(path: String) -> Self {
        Self { path, helpers: HashMap::new() }
    }

    pub fn add_helper(&mut self, name: String, helper: Box<dyn HelperDef + Sync + Send>) {
        self.helpers.insert(name, helper);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { path: ".".to_string(), helpers: Default::default() }
    }
}

struct Vars {
    path: PathBuf,
}

pub async fn start(config: Config) {
    let mut data: BTreeMap<String, Value> = BTreeMap::new();
    let mut templates = Handlebars::new();
    let mut registry = Handlebars::new();
    registry.set_strict_mode(true);
    
    build::gen_templates(&config.path, &mut templates);
    build::register_data(&config.path, &mut data);
    build::build_pages(&config.path, &templates).unwrap();
    
    // create Vars struct with path
    let vars = Vars { path: Path::new(&config.path).to_path_buf() };

    // add helpers to registry
    for helper in config.helpers {
        registry.register_helper(&helper.0, helper.1);
    }

    let _ = rocket::build()
        .manage(registry)
        .manage(data)
        .manage(vars)
        .mount("/", routes![ssr])
        .launch()
        .await
        .unwrap();
}
