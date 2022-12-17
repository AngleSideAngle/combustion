use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use gray_matter::{engine::TOML, Matter};
use handlebars::Handlebars;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    name: String,
    template: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TemplateData {
    data: Metadata,
    content: String,
}

/// compiles a file into its desired state when served based on file extension
pub trait FileCompiler {
    fn compile(&self, input: &Path, out: &mut PathBuf, registry: &Handlebars) -> io::Result<()>;
}

pub struct MarkdownCompiler;
pub struct DefaultCompiler;

impl FileCompiler for MarkdownCompiler {
    fn compile(&self, input: &Path, out: &mut PathBuf, registry: &Handlebars) -> io::Result<()> {
        // println!("{:?}", registry.)
        let text = fs::read_to_string(input)?;

        // parse front matter (toml)
        let matter = Matter::<TOML>::new();
        let res = matter.parse(&text);

        // parse text from content found by the front matter parser
        let parser = Parser::new_ext(&res.content, Options::all());
        let mut parsed_text = String::new();
        html::push_html(&mut parsed_text, parser);

        // template the file, if specified in front matter
        if let Some(data) = res.data.and_then(|raw| raw.deserialize::<Metadata>().ok()) {
            let data = TemplateData {
                data,
                content: parsed_text,
            };
            parsed_text = registry.render(&data.data.template, &data).unwrap();
            // println!("{:?}", data.template);
        }

        // write file
        out.set_extension("html");
        fs::create_dir_all(out.parent().unwrap())?;
        let mut out = File::create(out)?;
        out.write(parsed_text.as_bytes())?;

        Ok(())
    }
}

impl FileCompiler for DefaultCompiler {
    fn compile(&self, input: &Path, out: &mut PathBuf, _registry: &Handlebars) -> io::Result<()> {
        fs::create_dir_all(out.parent().unwrap())?;
        fs::copy(input, out)?;

        Ok(())
    }
}
