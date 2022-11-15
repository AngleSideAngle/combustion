use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use pulldown_cmark::{html, Parser, Options};
use yew::html as yew_html;

/// compiles a file into its desired state when served based on file extension
pub trait FileCompiler {
    fn compile(&self, input: &Path, output: &mut PathBuf) -> io::Result<()>;
}

pub struct MarkdownCompiler;

impl FileCompiler for MarkdownCompiler {
    fn compile(&self, input: &Path, output: &mut PathBuf) -> io::Result<()> {
        let text = fs::read_to_string(input)?;
        let parser = Parser::new_ext(&text, Options::all());
        let mut parsed_text = String::new();
        html::push_html(&mut parsed_text, parser);
        output.set_extension("html");

        let mut out = File::create(output)?;
        out.write(parsed_text.as_bytes())?;
        // out.write(yew_html!(parsed_text).);
        Ok(())
    }
}

