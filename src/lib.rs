use clap::Parser;
use handlebars::Handlebars;
use std::{
  error::Error,
  ffi::OsStr,
  fs::read_to_string,
  io::{stdin, BufRead},
  path::PathBuf,
};

/// Simple templating program based on Handlebars and written in Rust
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
  /// Template file to use.
  pub template: PathBuf,

  /// File to grab data from. If not supplied, will use environment variables.
  #[clap(short, long)]
  pub data: Option<PathBuf>,

  /// Output path. If empty, will output to stdout.
  #[clap(short, long)]
  pub output: Option<PathBuf>,

  /// Strict mode; errors when a value is in the template and can't be
  /// found in the data
  #[clap(short, long)]
  pub strict: bool,
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

  pub fn get_template_path_is_stdin(&self) -> bool {
    self.template.as_os_str() == OsStr::new("-")
  }

  pub fn register_template(&self, handlebars: &mut Handlebars) -> Result<(), Box<dyn Error>> {
    // if template file specified register it
    if !self.get_template_path_is_stdin() {
      handlebars.register_template_file("template", &self.template)?;
    }
    // else, collect stdin to string and register it
    else {
      let mut mah_string = String::new();
      let mut stdin_lock = stdin().lock();
      loop {
        if stdin_lock.read_line(&mut mah_string)? == 0 {
          break;
        }
      }
      // println!("stdin:\n{}", &mah_string);
      if let Err(x) = handlebars.register_template_string("template", mah_string) {
        return Err(format!(
          "Error parsing template from stdin. Make sure there are no syntax errors.\nError: {}",
          x.to_string()
        ))?;
      }
    }

    Ok(())
  }
}
