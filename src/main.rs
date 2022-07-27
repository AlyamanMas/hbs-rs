use clap::Parser;
use handlebars::Handlebars;
use hbs::Config;
use std::env::vars;
use std::{collections::BTreeMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
  let args: Config = Config::parse();

  let mut handlebars = Handlebars::new();
  if args.strict {
    handlebars.set_strict_mode(true);
  }

  let data = BTreeMap::from_iter(vars());

  args.register_template(&mut handlebars)?;

  println!(
    "Rendering template:\n{}",
    handlebars.render("template", &data)?
  );

  Ok(())
}
