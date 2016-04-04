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
		let mf_path = self.mapping_file_path(mapping);
		self.mappings.insert(mapping.clone(), Mapping::parse(mf_path.as_path()));
	}


	fn mapping_file_path(&self, mapping: &MappingSpec) -> PathBuf {
		self.ws_dir.join(format!("{}.zip", mapping))
	}
}
