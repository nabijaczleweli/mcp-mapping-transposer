extern crate csv;
extern crate zip;
extern crate curl;
extern crate clap;
extern crate regex;
extern crate bidir_map;

mod options;
mod network;
mod mapping;
mod workspace;

use options::Options;
use workspace::Workspace;


fn main() {
	let opts = Options::parse();

	let workspace = Workspace::prepare(&opts.data_dir);
	workspace.ensure_mapping_present(&opts.source_mapping);
	workspace.ensure_mapping_present(&opts.destination_mapping);
}
