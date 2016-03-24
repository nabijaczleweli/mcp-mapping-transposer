use std::path::Path;
use std::fs;
use network;

use mapping::MappingSpec;


pub struct Workspace<'a> {
	ws_dir: &'a Path,
}

impl<'a> Workspace<'a> {
	pub fn prepare<ToPath: AsRef<Path>>(ws_dir: &'a ToPath) -> Workspace {
		let ws_dir = ws_dir.as_ref();
		fs::create_dir_all(ws_dir).unwrap();

		Workspace{
			ws_dir: ws_dir,
		}
	}

	pub fn ensure_mapping_present(&self, mapping: &MappingSpec) {
		let file_path = self.ws_dir.join(format!("{}.zip", mapping));

		if !file_path.exists() {
			network::download_mapping(file_path.as_path(), mapping);
		}
	}
}
