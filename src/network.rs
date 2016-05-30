use curl::easy::Easy;
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
	let mut buf = Vec::new();

	let mut ctx = Easy::new();
	ctx.url(&url).unwrap();
	{
		let mut ctx = ctx.transfer();
		ctx.progress_function(|dltotal, dlnow, _, _| {
			if dltotal != 0f64 && dlnow != 0f64 {
				print!("\r{}%   ", ((dlnow / dltotal) * 10000f64).round() / 100f64);
			}

			true
		}).unwrap();
		ctx.write_function(|data| {
			buf.extend_from_slice(data);
			Ok(data.len())
		}).unwrap();
		ctx.perform().unwrap();
	}

	File::create(destination_file).unwrap().write_all(&buf).unwrap();
	println!("");
}
