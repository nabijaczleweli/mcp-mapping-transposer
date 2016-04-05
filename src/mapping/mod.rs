use bidir_map::BidirMap;
use std::path::Path;
use std::str::FromStr;
use std::fs::File;
use std::io;
use zip::ZipArchive;
use csv::Reader;

mod spec;

pub use self::spec::*;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Mapping {
	pub fields_seargename_name: BidirMap<String, String>,
	pub fields_seargename_desc: BidirMap<String, String>,
	pub methods_seargename_name: BidirMap<String, String>,
	pub methods_seargename_desc: BidirMap<String, String>,
	pub params_param_name: BidirMap<String, String>,
}

impl Mapping {
	pub fn parse(mapping_path: &Path) -> Mapping {
		let mut result = Mapping{
			fields_seargename_name: BidirMap::new(),
			fields_seargename_desc: BidirMap::new(),
			methods_seargename_name: BidirMap::new(),
			methods_seargename_desc: BidirMap::new(),
			params_param_name: BidirMap::new(),
		};

		let mut zip = ZipArchive::new(File::open(mapping_path).unwrap()).unwrap();
		for file_idx in 0..zip.len() {
			let mut file = zip.by_index(file_idx).unwrap();
			let file_kind: CsvKind = str::parse(file.name()).unwrap();

			let mut csv_reader = Reader::from_reader(file).has_headers(true).flexible(true);

			match file_kind {
				_ => (),
			}
		}

		result
	}
}


enum CsvKind {
	Fields,
	Methods,
	Params,
}

impl FromStr for CsvKind {
	type Err = io::Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"fields.csv"  => Ok(CsvKind::Fields),
			"methods.csv" => Ok(CsvKind::Methods),
			"params.csv"  => Ok(CsvKind::Params),
			_             => Err(io::Error::new(io::ErrorKind::InvalidData, "Not 'fields.csv', 'methods.csv' nor 'params.csv'")),
		}
	}
}
