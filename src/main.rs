use clap::Parser;
use handlebars::Handlebars;
use hbs::{Config, Data};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let args: Config = Config::parse();

  let mut handlebars = Handlebars::new();
  if args.strict {
    handlebars.set_strict_mode(true);
  }

  let data: Data = args.get_data();

  args.register_template(&mut handlebars)?;

  print!("{}", args.render(&data, &mut handlebars)?);

  Ok(())
}
