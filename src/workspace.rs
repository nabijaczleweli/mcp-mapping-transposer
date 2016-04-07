use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use network;

use mapping::{Mapping, MappingSpec};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace<'a> {
	ws_dir: &'a Path,
	mappings: HashMap<MappingSpec, Mapping>,
}

impl<'a> Workspace<'a> {
	pub fn prepare<ToPath: AsRef<Path>>(ws_dir: &'a ToPath) -> Workspace {
		let ws_dir = ws_dir.as_ref();
		fs::create_dir_all(ws_dir).unwrap();

		Workspace{
			ws_dir: ws_dir,
			mappings: HashMap::new(),
		}
	}

	pub fn ensure_mapping_present(&self, mapping: &MappingSpec) {
		let file_path = self.mapping_file_path(mapping);

		if !file_path.exists() {
			network::download_mapping(file_path.as_path(), mapping);
		}
	}

	pub fn decypher_mapping(&mut self, mapping: &MappingSpec) {
		println!("Loading {}...", mapping);
		let mf_path = self.mapping_file_path(mapping);
		self.mappings.insert(mapping.clone(), Mapping::parse(mf_path.as_path()));
	}

	pub fn lookup(&self, in_m: &MappingSpec, to_m: &MappingSpec, what: &String) -> String {
		let in_m = self.mappings.get(in_m).unwrap();
		let to_m = self.mappings.get(to_m).unwrap();

		if in_m.fields_seargename_name.contains_second_key(what) {
			let seargename = in_m.fields_seargename_name.get_by_second(what).unwrap();
			format!("field {} {} {}", seargename,
			                    to_m.fields_seargename_name.get_by_first(seargename).unwrap(),
				                  to_m.fields_seargename_desc.get_by_first(seargename).unwrap())
		} else if in_m.fields_seargename_desc.contains_second_key(what) {
			let seargename = in_m.fields_seargename_desc.get_by_second(what).unwrap();
			format!("field {} {} {}", seargename,
			                    to_m.fields_seargename_name.get_by_first(seargename).unwrap(),
			                    to_m.fields_seargename_desc.get_by_first(seargename).unwrap())
		} else if in_m.fields_seargename_name.contains_first_key(what) {
			format!("field {} {} {}", what, to_m.fields_seargename_name.get_by_first(what).unwrap(), to_m.fields_seargename_desc.get_by_first(what).unwrap())
		} else if in_m.methods_seargename_name.contains_second_key(what) {
			let seargename = in_m.methods_seargename_name.get_by_second(what).unwrap();
			format!("method {} {} {}", seargename,
			                           to_m.methods_seargename_name.get_by_first(seargename).unwrap(),
				                         to_m.methods_seargename_desc.get_by_first(seargename).unwrap())
		} else if in_m.methods_seargename_desc.contains_second_key(what) {
			let seargename = in_m.methods_seargename_desc.get_by_second(what).unwrap();
			format!("method {} {} {}", seargename,
			                           to_m.methods_seargename_name.get_by_first(seargename).unwrap(),
			                           to_m.methods_seargename_desc.get_by_first(seargename).unwrap())
		} else if in_m.methods_seargename_name.contains_first_key(what) {
			format!("method {} {} {}", what, to_m.methods_seargename_name.get_by_first(what).unwrap(), to_m.methods_seargename_desc.get_by_first(what).unwrap())
		} else if in_m.params_param_name.contains_second_key(what) {
			let seargename = in_m.params_param_name.get_by_second(what).unwrap();
			format!("param {} {}", seargename, to_m.params_param_name.get_by_first(seargename).unwrap())
		} else if in_m.params_param_name.contains_first_key(what) {
			format!("param {} {}", what, to_m.params_param_name.get_by_first(what).unwrap())
		} else {
			"NOTHING".to_string()
		}
	}


	fn mapping_file_path(&self, mapping: &MappingSpec) -> PathBuf {
		self.ws_dir.join(format!("{}.zip", mapping))
	}
}
