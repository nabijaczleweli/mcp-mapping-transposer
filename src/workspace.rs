use std::path::Path;
use std::fs;
use std::io;
use network;

use mapping::MappingSpec;


pub fn prepare<ToPath: AsRef<Path>>(ws_dir: &ToPath) {
	fs::create_dir_all(ws_dir.as_ref()).unwrap();
}

pub fn ensure_mapping_present<ToPath: AsRef<Path>>(ws_dir: &ToPath, mapping: &MappingSpec) -> io::Result<()> {
	let file_path = ws_dir.as_ref().join(format!("{}.zip", mapping));

	if !file_path.exists() {
		network::download_mapping(file_path.as_path(), mapping);
	}

	Ok(())
}
