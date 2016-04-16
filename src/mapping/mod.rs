use bidir_map::BidirMap;
use std::thread::{self, JoinHandle};
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
		let mut fields_handle: Option<JoinHandle<(BidirMap<String, String>, BidirMap<String, String>)>> = None;
		let mut methods_handle: Option<JoinHandle<(BidirMap<String, String>, BidirMap<String, String>)>> = None;
		let mut params_handle: Option<JoinHandle<BidirMap<String, String>>> = None;

		let mut zip = ZipArchive::new(File::open(mapping_path).unwrap()).unwrap();
		for file_idx in 0..zip.len() {
			let file = zip.by_index(file_idx).unwrap();
			let file_kind: CsvKind = str::parse(file.name()).unwrap();

			match file_kind {
				CsvKind::Fields => fields_handle = Some(spawn_double_reader(mapping_path, file_idx)),
				CsvKind::Methods => methods_handle = Some(spawn_double_reader(mapping_path, file_idx)),
				CsvKind::Params => params_handle = Some(spawn_single_reader(mapping_path, file_idx)),
			}
		}

		let (fields_seargename_name, fields_seargename_desc) = match fields_handle {
			Some(handle) => handle.join().unwrap(),
			None         => (BidirMap::new(), BidirMap::new()),
		};
		let (methods_seargename_name, methods_seargename_desc) = match methods_handle {
			Some(handle) => handle.join().unwrap(),
			None         => (BidirMap::new(), BidirMap::new()),
		};
		let params_param_name = match params_handle {
			Some(handle) => handle.join().unwrap(),
			None         => BidirMap::new(),
		};
		Mapping{
			fields_seargename_name: fields_seargename_name,
			fields_seargename_desc: fields_seargename_desc,
			methods_seargename_name: methods_seargename_name,
			methods_seargename_desc: methods_seargename_desc,
			params_param_name: params_param_name,
		}
	}
}


fn spawn_double_reader(path: &Path, file_idx: usize) -> JoinHandle<(BidirMap<String, String>, BidirMap<String, String>)> {
	let file = File::open(path).unwrap();
	thread::spawn(move || {
		let mut zip = ZipArchive::new(file).unwrap();
		let mut csv_reader = Reader::from_reader(zip.by_index(file_idx).unwrap()).has_headers(true).flexible(true);

		let mut name = BidirMap::new();
		let mut desc = BidirMap::new();

		for row in csv_reader.records() {
			let row = row.unwrap();
			name.insert(row[0].clone(), row[1].clone());
			desc.insert(row[0].clone(), row[3].clone());
		}

		(name, desc)
	})
}

fn spawn_single_reader(path: &Path, file_idx: usize) -> JoinHandle<BidirMap<String, String>> {
	let file = File::open(path).unwrap();
	thread::spawn(move || {
		let mut zip = ZipArchive::new(file).unwrap();
		let mut csv_reader = Reader::from_reader(zip.by_index(file_idx).unwrap()).has_headers(true).flexible(true);

		let mut name = BidirMap::new();

		for row in csv_reader.records() {
			let row = row.unwrap();
			name.insert(row[0].clone(), row[1].clone());
		}

		name
	})
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
