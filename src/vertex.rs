#![allow(dead_code)]

use std::cmp;

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
pub struct Vertex {
	pub x: i32,
	pub y: i32,
}

impl Vertex {
	pub fn new(x: i32, y: i32) -> Vertex {
		Vertex { x: x, y: y }
	}

	#[inline]	
	pub fn distance(&self, goal: &Vertex) -> i32 {
		(10f32 * (((self.x - goal.x).pow(2) + (self.y - goal.y).pow(2)) as f32).sqrt()) as i32
	}

	#[inline]
	pub fn hdistance(&self, goal: &Vertex) -> i32 {
		10 * cmp::max((self.x - goal.x).abs(), (self.y - goal.y).abs())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn constructor_works() {
		assert_eq!(Vertex::new(1, 1), Vertex { x: 1, y: 1 });
		assert_eq!(Vertex::new(-1, 2), Vertex { x: -1, y: 2 });
		assert_eq!(Vertex::new(-1, -2), Vertex { x: -1, y: -2 });
	}

	#[test]
	fn distance_works() {
		let (p1, p2) = (Vertex::new(1, 2), Vertex::new(4, 3));
		assert_eq!(p1.distance(&p2), 31);
		let (p3, p4) = (Vertex::new(0, 0), Vertex::new(-1, -1));
		assert_eq!(p3.distance(&p4), 14);
	}

	#[test]
	fn hdistance_works() {
		let (p1, p2) = (Vertex::new(1, 2), Vertex::new(4, 3));
		assert_eq!(p1.hdistance(&p2), 30);
		let (p3, p4) = (Vertex::new(0, 0), Vertex::new(-1, -1));
		assert_eq!(p3.hdistance(&p4), 10);
	}
}
