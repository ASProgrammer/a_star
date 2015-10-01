mod vertex;
mod node;
mod file_reader;
mod astar;

use file_reader::FileReader;
use std::path::Path;
use astar::AStar;
use vertex::Vertex;

fn print_path(path: &Vec<Vertex>) {
	println!("Minimal path: ");
	for point in 0..path.len() {
		println!("{:?}", path[path.len() - point -1]);
	}
}

fn main() {
	let path = Path::new("test_files").join("test4");
	
	let mut reader = match FileReader::new(&path) {
		Ok(reader) => { reader },
		Err(e) => { 
			println!("{}", e);
			return
		 }
	};

	let map = match reader.read() {
		Ok(val) => { val },	
		Err(_) => { 
			println!("Cannot read graph from file");
			return
		}
	};
	
	match AStar::find_path(&map, Vertex::new(0, 0), Vertex::new(3, 3)) {
		Some(path) => {
			print_path(&path);
		},
	
		_ =>  { 
			println!("Cannot find minimal path!\n");
		}	
	}
}
