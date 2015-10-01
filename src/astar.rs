#![allow(dead_code)]

use node::Node;
use vertex::Vertex;
use std::collections::{HashMap, BinaryHeap};
use std::collections::{LinkedList};

pub struct AStar;

pub type GraphHash = HashMap<Vertex, Vec<Vertex>>;

impl AStar {
	fn get_nodes_list(graph: &GraphHash) -> Vec<Node> {
		let mut vec_nodes = Vec::new();
		for key in graph.keys() {
			vec_nodes.push(Node::new(*key));
		}
		vec_nodes
	}
	
	fn get_ref_on_node<'a>(node: &'a Node, vec: &'a Vec<Node>) -> Option<&'a Node> {
		for item in vec {
			if item.pos() == node.pos() {
				return Some(item);
			}
		}
		None
	}

	fn get_copy(vert1: &Vertex, vec: &Vec<Node>) -> Option<Node> {
		for node in vec {
			if node.pos() == *vert1 {
				let mut res =  Node::new(*vert1);
				res.property = node.property;
				res.prev = node.prev;
				return Some(res);
			}
		}
		None
	}

	fn synchronize(node: &Node, vec: &mut Vec<Node>) {
		for item in vec.iter_mut() {
			if item.pos() == node.pos() {
				item.property = node.property;
				item.prev = node.prev;
			}
		}
	}
	 
	fn reconstruction_path(start: &Node) -> Vec<Vertex> {
		let mut ptr = start.prev;
		let mut path = Vec::new();
		path.push(start.pos());
		while !ptr.is_null() {
			let node = unsafe { &*ptr }; 
			path.push(node.pos());
			ptr = node.prev;
		}
		path
	}	

	pub fn find_path(graph: &GraphHash, start: Vertex, goal: Vertex) -> Option<Vec<Vertex>> {
		let mut vec = AStar::get_nodes_list(graph);	
		let mut st = AStar::get_copy(&start, &vec).unwrap();	

		st.update(0i32, start.hdistance(&goal));
		AStar::synchronize(&st, &mut vec);
	
		let mut open_set = BinaryHeap::new();
		let mut close_set = LinkedList::new();

		open_set.push(st);
		while let Some(x) = open_set.pop() {
			if x.pos() == goal {
				return Some(AStar::reconstruction_path(&x));  
			}
			close_set.push_back(x.pos());

			let vec_smeg = graph.get(&x.pos()).unwrap();

			for y in vec_smeg {
				if let Some(_) = close_set.iter().find(|ref_vert| **ref_vert == *y) {
					continue;
				}

				let mut y_node = AStar::get_copy(y, &vec).unwrap();
				let new_g_score = x.get_g() + x.pos().distance(y);
				let mut is_better = false;
				if let None = open_set.iter().find(|ref_node| ref_node.pos() == *y) {
					is_better = true;
				}
				else if new_g_score < y_node.get_g() {
					is_better = true;
				}

				if is_better {
					y_node.prev = AStar::get_ref_on_node(&x, &vec).unwrap();
					y_node.update(new_g_score, y.hdistance(&goal));
					AStar::synchronize(&y_node, &mut vec);
				}
		
				if let None = open_set.iter().find(|ref_node| ref_node.pos() == *y) {
					open_set.push(y_node);
				}
			}
		}
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::HashMap;
	use vertex::Vertex;	
	use file_reader::FileReader;
	use std::path::Path;

	#[test]
	fn find_path_works1() {
		let mut map = HashMap::new();
		map.insert(Vertex::new(0, 0), vec![]);
	
		if let Some(_) = AStar::find_path(&map, Vertex::new(0, 0), Vertex::new(1, 1)) {
			panic!();
		}
	}
	
	#[test]
	fn find_path_works2() {
		let mut map = HashMap::new();
		map.insert(Vertex::new(0, 0), vec![]);
		
		if let None = AStar::find_path(&map, Vertex::new(0, 0), Vertex::new(0, 0)) {
			panic!();
		}
	}

	#[test]
	fn find_path_works3() {
		let path = Path::new("src").join("test_files").join("test4");
		let mut file = match FileReader::new(&path) {
			Ok(file) => { file },
			_ => { panic!(); return; }
		};
	
		let map = file.read().unwrap();
		if let Some(_) = AStar::find_path(&map, Vertex::new(0, 0), Vertex::new(5, 2)) {
			panic!();
		}	
	
		let correct_path = vec![
			Vertex::new(0, 0), Vertex::new(0, 1), Vertex::new(0, 2),
			Vertex::new(1, 2), Vertex::new(1, 3), Vertex::new(2, 3),
			Vertex::new(3, 3)
		];

		let finded_path = AStar::find_path(&map, Vertex::new(0, 0), Vertex::new(3, 3)).unwrap();
		for node in finded_path {
			if let None  = correct_path.iter().find(|ref_node| **ref_node == node) {
				panic!();
			}
		}
	}
}
