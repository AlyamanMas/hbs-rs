use clap::Parser;
use handlebars::Handlebars;
use std::{
  collections::BTreeMap,
  env::vars,
  error::Error,
  ffi::{OsStr, OsString},
  fs::{self, read_to_string},
  io::{stdin, BufRead},
  path::PathBuf,
};

/// Simple templating program based on Handlebars and written in Rust
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
  /// Template file to use.
  pub template: PathBuf,

  // TODO: add ability to read data from stdin when file is specified as non-stdin
  /// File to grab data from. If not supplied, will use environment variables.
  /// Currently supports reading from JSON, YAML, and TOML.
  #[clap(short, long)]
  pub data: Option<PathBuf>,

  // TODO: implement writing to output
  /// Output path. If empty, will output to stdout.
  #[clap(short, long)]
  pub output: Option<PathBuf>,

  /// Strict mode; errors when a value is in the template and can't be
  /// found in the data. Also disables reading data from environment when
  /// data file can't be parsed.
  #[clap(short, long)]
  pub strict: bool,
  // TODO: add watch mode which waites for file changes and rerenders
  // automatically
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

  fn serialize_data(&self, string: &str, file_extension: &str) -> Data {
    fn serialize_internal(
      strict: bool,
      string: &str,
      file_extension: &str,
      nbr_of_tries: u8,
    ) -> Data {
      if nbr_of_tries == 3 {
        if strict {
          panic!("Couldn't parse file in any format and strict mode is on");
        } else {
          eprintln!("Couldn't parse file in any format, but strict mode is not on, so using environment variables instead");
          return Data::Env(BTreeMap::from_iter(vars()));
        }
      }
      match file_extension {
        "yaml" | "yml" => match serde_yaml::from_str(string) {
          Ok(x) => Data::Yaml(x),
          Err(x) => {
            eprintln!("Error, couldn't deserialize file as yaml, trying json\nError: {x}");
            serialize_internal(strict, string, "json", nbr_of_tries + 1)
          }
        }, // end yaml
        "json" => match serde_json::from_str(string) {
          Ok(x) => Data::Json(x),
          Err(x) => {
            eprintln!("Error, couldn't deserialize file as json, trying toml\nError: {x}");
            serialize_internal(strict, string, "toml", nbr_of_tries + 1)
          }
        }, // end json
        "toml" => match toml::from_str(string) {
          Ok(x) => Data::Toml(x),
          Err(x) => {
            eprintln!("Error, couldn't deserialize file as toml, trying yaml\nError: {x}");
            serialize_internal(strict, string, "yaml", nbr_of_tries + 1)
          }
        }, // end toml
        // TODO: switch to using `Option` instead of implicit ""
        "" => Data::Env(BTreeMap::from_iter(vars())),
        _ => {
          eprintln!("File extension `{file_extension}` not known, trying deserialization as yaml");
          serialize_internal(strict, string, "yaml", 0)
        }
      } // end match file_extension
    } // end serialize_internal

    serialize_internal(self.strict, string, file_extension, 0)
  } // end serialize_data

  pub fn get_data(&self) -> Data {
    let file_content: Option<String> = match &self.data {
      Some(x) => match fs::read_to_string(x) {
        Ok(y) => Some(y),
        Err(y) => {
          eprintln!(
            "Error while trying to read file {}: {y}",
            self.data.as_ref().unwrap().display()
          );
          None
        }
      },
      None => None,
    };

    let file_extension: Option<&OsStr> = match &self.data {
      Some(x) => x.extension(),
      None => None,
    };

    if let Some(content) = file_content {
      self.serialize_data(
        &content,
        file_extension
          .unwrap_or(&OsString::from(""))
          .to_str()
          .unwrap(),
      )
    } else {
      Data::Env(BTreeMap::from_iter(vars()))
    }
  }
}

pub enum Data {
  Json(serde_json::Value),
  Yaml(serde_yaml::Value),
  Toml(toml::Value),
  Env(BTreeMap<String, String>),
}
