extern crate zip;
extern crate curl;
extern crate clap;

mod options;

use options::Options;


fn main() {
	let opts = Options::parse();
	println!("{:?}", opts);
}
