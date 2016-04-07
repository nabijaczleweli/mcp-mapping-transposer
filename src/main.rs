extern crate csv;
extern crate zip;
extern crate curl;
extern crate clap;
extern crate regex;
extern crate bidir_map;

mod trans;
mod options;
mod network;
mod mapping;
mod workspace;

use trans::translate;
use options::Options;
use workspace::Workspace;


fn main() {
	let opts = Options::parse();

	let mut workspace = Workspace::prepare(&opts.data_dir);
	workspace.ensure_mapping_present(&opts.source_mapping);
	workspace.ensure_mapping_present(&opts.destination_mapping);
	workspace.decypher_mapping(&opts.source_mapping);
	workspace.decypher_mapping(&opts.destination_mapping);
	println!("");

	if let Err(error) = translate(&workspace, &opts.source_mapping, &opts.destination_mapping) {
			println!("Translation I/O error: {}", error);
	}
}
