use std::path::Path;
use std::fs;
use std::io;


pub fn prepare<ToPath: AsRef<Path>>(ws_dir: &ToPath) {
	fs::create_dir_all(ws_dir.as_ref()).unwrap();
}

pub fn ensure_mapping_present<ToPath: AsRef<Path>>(ws_dir: &ToPath, mapping: &String) -> io::Result<()> {
	let file_path = ws_dir.as_ref().join(format!("{}.zip", mapping));

	if !file_path.exists() {
		// Download mapping
	}

	Ok(())
}
