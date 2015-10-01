#![allow(dead_code)]

use vertex::Vertex;
use std::cmp::Ordering;
use std::ptr;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Node {
	pos: Vertex,
	pub property: (i32, i32, i32),
	pub prev: *const Node,
}

impl Node {
	pub fn new(pos: Vertex) -> Node {
		Node { pos: pos, property: (0i32, 0i32, 0i32), prev: ptr::null() }	
	}
	
	#[inline]
	pub fn pos(&self) -> Vertex {
		self.pos
	}
	
	#[inline]
	pub fn update(&mut self, g: i32, h: i32) {
		self.property = (g + h, g, h);
	}
	
	#[inline]
	pub fn get_g(&self) -> i32 {
		self.property.1
	}
}

impl Ord for Node {
	fn cmp(&self, other: &Node) -> Ordering {
		other.property.0.cmp(&self.property.0)
	}
}

impl PartialOrd for Node {
	fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use vertex::Vertex;
	
	#[test]
	fn pos_works() {
		let mut node = Node::new(Vertex::new(0, 0));
		assert_eq!(Vertex::new(0, 0), node.pos());
		
		node.pos = Vertex::new(1, -1);
		assert_eq!(Vertex::new(1, -1), node.pos());
	}

	#[test]
	fn update_works() {
		let mut node = Node::new(Vertex::new(0, 0));
		assert_eq!(node.property.0, 0);
		
		node.update(12, 43);
		assert_eq!(node.property.0, 55);
	}

	#[test]
	fn get_g_works() {
		let mut node = Node::new(Vertex::new(0, 0));
		assert_eq!(node.get_g(), 0);
	
		node.update(12, 4);
		assert_eq!(node.get_g(), 12);
	}
}
