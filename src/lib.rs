use clap::Parser;
use std::{error::Error, fs::read_to_string, path::PathBuf};

// TODO: add support for taking input from stdin
// TODO: add option for strict mode with `handlebars.set_strict_mode(true)`
/// Simple templating program based on Handlebars
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
  /// Template file to use.
  #[clap(value_parser)]
  pub template: PathBuf,

  /// File to grab data from. If not supplied, will use environment variables.
  #[clap(short, long, value_parser)]
  pub data: Option<PathBuf>,

  /// Output path. If empty, will output to stdout.
  #[clap(short, long, value_parser)]
  pub output: Option<PathBuf>,
}

impl Config {
  pub fn get_template_string(&self) -> Result<String, std::io::Error> {
    read_to_string(&self.template)
  }

  pub fn get_data_string(&self) -> Result<String, Box<dyn Error>> {
    if let Some(x) = self.data.as_ref() {
      return Ok(read_to_string(x)?);
    }
    Err(format!("Data file not supplied"))?
  }
}
