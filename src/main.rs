extern crate zip;
extern crate curl;
extern crate clap;

mod options;
mod workspace;

use options::Options;


fn main() {
	let opts = Options::parse();
	println!("{:?}", opts);

	workspace::prepare(&opts.data_dir);
	workspace::ensure_mapping_present(&opts.data_dir, &opts.source_mapping).unwrap();
}
