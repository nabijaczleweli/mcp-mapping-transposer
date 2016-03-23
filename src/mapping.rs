use regex::Regex;
use std::str::FromStr;
use std::io;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MappingSpec {
	pub minecraft_version: String,
	pub stability: MappingStability,
	pub mapping_id: u32,
}

impl MappingSpec {
	fn from_arg_value(arg: &str) -> MappingSpec {
		let captures = Regex::from_str(r"([[:digit:]]\.[[:digit:]](?:\.[[:digit:]])?)-(stable|snapshot)_([[:digit:]]+)").unwrap().captures(arg).unwrap();

		MappingSpec{
			minecraft_version: captures.at(1).unwrap().to_string(),
			stability: str::parse(captures.at(2).unwrap()).unwrap(),
			mapping_id: str::parse(captures.at(3).unwrap()).unwrap(),
		}
	}
}


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum MappingStability {
	Stable,
	Snapshot,
}

impl ToString for MappingStability {
	fn to_string(&self) -> String {
		match self {
			&MappingStability::Stable   => "stable",
			&MappingStability::Snapshot => "snapshot",
		}.to_string()
	}
}

impl FromStr for MappingStability {
	type Err = io::Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"stable"   => Ok(MappingStability::Stable),
			"snapshot" => Ok(MappingStability::Snapshot),
			_          => Err(io::Error::new(io::ErrorKind::InvalidData, "Not 'stable' nor 'snapshot'")),
		}
	}
}
