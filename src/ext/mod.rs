use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf}, default,
};

use pulldown_cmark::{html, Options, Parser};

/// compiles a file into its desired state when served based on file extension
pub trait FileCompiler {
    fn compile(&self, input: &Path, out: &mut PathBuf) -> io::Result<()>;
}

pub struct MarkdownCompiler;
pub struct DefaultCompiler;

impl FileCompiler for MarkdownCompiler {
    fn compile(&self, input: &Path, out: &mut PathBuf) -> io::Result<()> {
        println!("compiling md");
        let text = fs::read_to_string(input)?;
        println!("hi");

        // parse text
        let parser = Parser::new_ext(&text, Options::all());
        let mut parsed_text = String::new();
        html::push_html(&mut parsed_text, parser);
        
        out.set_extension("html");
        
        
        println!("target dir? {:?}", out.parent().unwrap());
        fs::create_dir_all(out.parent().unwrap())?;
        let mut out = File::create(out)?;
        out.write(parsed_text.as_bytes())?;
        
        Ok(())
    }
}

impl FileCompiler for DefaultCompiler {
    fn compile(&self, input: &Path, out: &mut PathBuf) -> io::Result<()> {
        fs::create_dir_all(out.parent().unwrap())?;
        fs::copy(input, out)?;

        Ok(())
    }
}