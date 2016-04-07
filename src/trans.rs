use std::io::{self, BufRead};

use mapping::MappingSpec;
use workspace::Workspace;


pub fn translate(wspc: &Workspace, src: &MappingSpec, dst: &MappingSpec) -> Result<(), io::Error> {
	let stdin = io::stdin();
	for line in stdin.lock().lines() {
		let line = match line {
			Ok(s) => s,
			Err(err) => return Err(err),
		};
		if line.contains('\u{4}') {
			break;
		}

		//
	}
	Ok(())
}
