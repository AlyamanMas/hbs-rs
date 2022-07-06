use clap::Parser;
use std::path::PathBuf;

/// Simple templating program based on Handlebars
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  /// Template file to use.
  #[clap(value_parser)]
  template: PathBuf,

  /// File to grab data from. If not supplied, will use environment variables.
  #[clap(short, long, value_parser)]
  data: Option<PathBuf>,

  /// Output path. If empty, will output to stdout.
  #[clap(short, long, value_parser)]
  output: Option<PathBuf>,
}

fn main() {
  let args: Args = Args::parse();
}
