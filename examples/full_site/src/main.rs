use combustion::Config;

use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext,
};
use rocket::fs::relative;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut config = Config::new(relative!("").to_string());
    config.add_helper("thing".to_string(), Box::new(thing));
    // config.
    combustion::start(config).await;

    Ok(())
}

fn thing(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();

    out.write("Example helper usage\nFor the helper, check out examples/full_site/main.rs")?;
    let x = 1; // let's imagine some data was fetched from a database here
    out.write(&format!("your value is: {}", x))?;

    out.write(param.value().render().as_ref())?;

    Ok(())
}
