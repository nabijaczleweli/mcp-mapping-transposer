use curl::http;
use std::path::Path;
use std::io::Write;
use std::fs::File;

use mapping::MappingSpec;


pub fn download_mapping(destination_file: &Path, mapping: &MappingSpec) {
	println!("Downloading {}...", mapping);
	download_to_file(destination_file, format!("http://export.mcpbot.bspk.rs/mcp_{0}/{1}-{2}/mcp_{0}-{1}-{2}.zip",
		                                         mapping.stability, mapping.mapping_id, mapping.minecraft_version));
}

pub fn download_to_file(destination_file: &Path, url: String) {
	File::create(destination_file).unwrap().write_all(http::handle().get(url).progress(percent_progress_handler).exec().unwrap().get_body()).unwrap();
	println!("");
}


fn percent_progress_handler(dltotal: usize, dlnow: usize, _: usize, _: usize) {
	if dltotal != 0 && dltotal != 0 {
		print!("\r{}%   ", ((dlnow as f64 / dltotal as f64) * 10000f64).round() / 100f64);
	}
}
