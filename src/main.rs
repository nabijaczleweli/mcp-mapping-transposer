extern crate zip;
extern crate curl;
extern crate clap;
extern crate regex;

mod options;
mod network;
mod mapping;
mod workspace;

use options::Options;


fn main() {
	let opts = Options::parse();

	workspace::prepare(&opts.data_dir);
	workspace::ensure_mapping_present(&opts.data_dir, &opts.source_mapping).unwrap();
	workspace::ensure_mapping_present(&opts.data_dir, &opts.destination_mapping).unwrap();
}
