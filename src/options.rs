use clap::{App as Clapp, Arg as Clarg};
use std::path::{Path, PathBuf};
use std::env::home_dir;
use std::str::FromStr;
use regex::Regex;

use mapping::MappingSpec;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
	pub source_mapping: MappingSpec,
	pub destination_mapping: MappingSpec,
	pub data_dir: PathBuf,
}

impl Options {
	pub fn parse() -> Options {
		static USAGE: &'static str = "--data-dir [data-dir] 'Directory to store data in, default: $HOME/.mcp-mapping-transposer/'";

		let matches = Clapp::new("mcp-mapping-transposer").version(env!("CARGO_PKG_VERSION"))
			                                                .author("nabijaczleweli <nabijaczleweli@gmail.com>")
			                                                .about("Translate between MCP mappings")
			                                                .args_from_usage(USAGE)
			                                                .arg(Clarg::from_usage(
			                                                      "<mapping-from> 'The mapping to translate from. Format: <stable|snapshot>-<ID>-<MC_VER>'"
			                                                     ).validator(Options::mapping_validator))
			                                                .arg(Clarg::from_usage(
			                                                      "<mapping-to> 'The mapping to translate to. Format: <stable|snapshot>-<ID>-<MC_VER>'"
			                                                     ).validator(Options::mapping_validator))
			                                                .get_matches();

		Options{
			source_mapping: MappingSpec::from_arg_value(matches.value_of("mapping-from").unwrap()),
			destination_mapping: MappingSpec::from_arg_value(matches.value_of("mapping-to").unwrap()),
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


	fn mapping_validator(arg: String) -> Result<(), String> {
		let validator_regex = Regex::from_str(r"(stable|snapshot)-([[:digit:]]+)-([[:digit:]]\.[[:digit:]](?:\.[[:digit:]])?)").unwrap();

		if validator_regex.is_match(&*&arg) {
			Ok(())
		} else {
			Err("Argument is not in the format '<stable|snapshot>-<ID>-<MC_VER>'".to_string())
		}
	}
}
