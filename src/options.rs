use clap::App as Clapp;
use std::env::home_dir;
use std::path::{Path, PathBuf};


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
	pub source_mapping: String,
	pub destination_mapping: String,
	pub data_dir: PathBuf,
}

impl Options {
	pub fn parse() -> Options {
		static USAGE: &'static str = "<mapping-from>        'The mapping to translate from'
		                              <mapping-to>          'The mapping to translate to'
		                              --data-dir [data-dir] 'Directory to store data in, default: $HOME/.mcp-mapping-transposer/'";

		let matches = Clapp::new("mcp-mapping-transposer").version(env!("CARGO_PKG_VERSION"))
			                                                .author("nabijaczleweli <nabijaczleweli@gmail.com>")
			                                                .about("Translate between MCP mappings")
			                                                .args_from_usage(USAGE)
			                                                .get_matches();

		Options{
			source_mapping: matches.value_of("mapping-from").unwrap().to_string(),
			destination_mapping: matches.value_of("mapping-to").unwrap().to_string(),
			data_dir: match matches.value_of("data-dir") {
			          	Some(data_dir) => Path::new(data_dir).to_path_buf(),
			          	None           => {
			          		let mut hd = home_dir().expect("Couldn't detect your home dir. Specify `data_dir` yourself.");
			          		hd.push(".mcp-mapping-transposer/");
			          		hd
			          	},
			          },
		}
	}
}
