#![allow(dead_code)]

use std::io::{self, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use vertex::Vertex;

#[derive(Debug)]
pub struct FileReader {
	file: File,
}

impl FileReader {
	pub fn new(path: &Path) -> io::Result<FileReader> {
		Ok(FileReader { file: try!(File::open(path)) })
	}

	#[inline]
	fn create_error(&self, msg: &str) -> io::Error {
		io::Error::new(ErrorKind::Other, msg)
	}

	#[inline]
	fn get_size_fields(&self, s: &str) -> Option<(i32, i32)> {
		let vec_sizes: Vec<&str> = s.split_whitespace().collect();
		if vec_sizes.len() != 2 { return None; }

		if let Ok(rs) = vec_sizes[0].parse::<i32>() {
			if let Ok(cs) = vec_sizes[1].parse::<i32>() {
				return Some((rs, cs));
			}
		}
		None
	}
	
	#[inline]
	fn is_set(&self, pos: (i32, i32), vec: &Vec<Vertex>) -> bool {
		let vert = Vertex::new(pos.0, pos.1);
		if let Some(_) = vec.iter().find(|ref_val| **ref_val == vert) {
			return true;
		}
		false
	}

	fn get_vector_smeg(&self, pos: (i32, i32), sizes: (i32, i32), vec: &Vec<Vertex>) -> Vec<Vertex> {
		let mut smeg_vec = Vec::new();
		if 0 < pos.0 {
			if !self.is_set((pos.0 - 1, pos.1), vec) {	
				smeg_vec.push(Vertex::new(pos.0 - 1, pos.1));
			}
		}
		if pos.0 < sizes.1 - 1 {
			if !self.is_set((pos.0 + 1, pos.1), vec) {
				smeg_vec.push(Vertex::new(pos.0 + 1, pos.1));
			}
		}
		if pos.1 > 0 {
			if !self.is_set((pos.0, pos.1 - 1), vec) {
				smeg_vec.push(Vertex::new(pos.0, pos.1 - 1));
			}
		}
		if pos.1 < sizes.0 - 1 {
			if !self.is_set((pos.0, pos.1 + 1), vec) {
				smeg_vec.push(Vertex::new(pos.0, pos.1 + 1));
			}
		}
		smeg_vec
	}

	fn get_vec_walls(&self, lines: &[&str]) -> Vec<Vertex> {
		let mut res = Vec::new();
		for i in 0..lines.len() {
			let vec_node: Vec<&str> = lines[i].split_whitespace().collect();
			for j in 0..vec_node.len() {
				if vec_node[j] == "0" { res.push(Vertex::new(j as i32, i as i32)); }
			}
		}
		return res;
	}
	
	fn get_map_without_walls(&self, sizes: (i32, i32), walls: &Vec<Vertex>) -> HashMap<Vertex, Vec<Vertex>> {
		let mut map = HashMap::new();
		for i in 0..sizes.0 {
			for j in 0..sizes.1 {
				let key = Vertex::new(j, i);
				if let Some(_) = walls.iter().find(|ref_vert| **ref_vert == key) {
					continue;
				}
				map.insert(key, self.get_vector_smeg((j, i), sizes, walls));
			}
		}
		map
	}

	pub fn read(&mut self) -> io::Result<HashMap<Vertex, Vec<Vertex>>> {
		let mut tmp_str = String::new();
		try!(self.file.read_to_string(&mut tmp_str));
		
		let lines: Vec<&str> = tmp_str.lines().collect();
		if lines.len() < 1 { return Err(self.create_error("Bad data")); }

		let (rs, cs) = self.get_size_fields(lines[0]).unwrap();
		let vec_walls = self.get_vec_walls(&lines[1..]);
	
		Ok(self.get_map_without_walls((rs, cs), &vec_walls))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::path::Path;
	use std::collections::HashMap;
	use vertex::Vertex;	

	#[test]
	fn constructor_works() {
		let path = Path::new("notexists");
		if let Ok(_) = FileReader::new(&path) {
			assert!(false);
		}
	}
		
	#[test]
	fn read1_works() {
		let path = Path::new("src").join("test_files").join("test1");
		let mut file = match FileReader::new(&path) {
			Ok(val) => { val },
			_ => { assert!(false); return; }
		};

		if let Ok(_) = file.read() {
			assert!(false);
		}
	}

	#[test]
	fn read2_works() {
		let path = Path::new("src").join("test_files").join("test2");
		let mut file = match FileReader::new(&path) {
			Ok(val) => { val },
			_ => { assert!(false); return; }
		};

		if let Err(_) = file.read() {
			assert!(false);
		}
	}

	#[test]
	fn read3_works() {
		let path = Path::new("src").join("test_files").join("test3");
		let mut file = match FileReader::new(&path) {
			Ok(val) => { val },
			_ => { assert!(false); return; }
		};

		let map = file.read().unwrap();
		let mut test_map = HashMap::new();
		test_map.insert(Vertex::new(0, 0), vec![Vertex::new(1, 0)]);
		test_map.insert(Vertex::new(1, 0), vec![Vertex::new(0, 0), Vertex::new(1, 1)]);
		test_map.insert(Vertex::new(1, 1), vec![Vertex::new(1, 0)]);
	
		assert_eq!(map, test_map);	
	}
}	
