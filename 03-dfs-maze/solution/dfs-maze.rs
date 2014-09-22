use std::rand::{Rng,task_rng};
use std::clone::Clone;
use std::ops::Add;


#[deriving(Clone,Show)]
struct Point {
	x: int,
	y: int
}

impl Point {
	fn new(x: int, y: int) -> Point {
		Point {x: x, y: y}
	}

	fn clamp(&mut self, xmin: int, xmax: int, ymin: int, ymax: int)
	{
		// Clamp (confine) the point to given dimensions
		if self.x < xmin { self.x = xmin }
		else if self.x > xmax { self.x = xmax }
		if self.y < ymin { self.y = ymin }
		else if self.y > ymax { self.y = ymax }
	}

	fn within(&self, x0: int, y0: int, x1: int, y1: int) -> bool {
		// Test whether the point is in a given area
		self.x >= x0 && self.y >= y0 && self.x <= x1 && self.y <= y1
	}
}

impl Add<Point, Point> for Point {
	// Implement addition of points
	fn add(&self, rhs: &Point) -> Point {
		Point{ x: self.x + rhs.x, y: self.y + rhs.y }
	}
}


static dirs: [Point, ..4] = [Point{x: 1, y: 0}, Point{x: -1,y:  0},
                             Point{x: 0, y: 1}, Point{x: 0, y: -1}];  // Possible directions of movement (represented by Points)


struct Maze {
	maze: Vec<Vec<bool>>,  // The maze itself is represented by vector of vectors of bools (a grid)
	                       // true is "path", false is "wall"
	width: uint,
	height: uint,
}

impl Maze {
	fn new(width: uint, height: uint) -> Maze {
		Maze {
			maze: Vec::from_fn(height, |_i|
			      	Vec::from_fn(width, |_j| false) ),   // Initializes the maze to widht*height fields of false
			width: width,
			height: height,
		}
	}

	fn randomize(&mut self, startx: int, starty: int) {
		let mut start = Point::new(startx, starty);
		start.clamp(1, self.width as int - 2, 1, self.height as int - 2);

		// To perform the DFS, a stack is needed, which will hold a Point for each spot to process
		// as well as another point which connects the last one with the current one, so that a path can be made.
		let mut stack: Vec<(Point, Point)> = Vec::new();
		stack.push((start, start));

		while !stack.is_empty() {
			let (p, wall) = stack.pop().unwrap();  // Stack is not empty, safe to unwrap

			if self.maze[p.y as uint][p.x as uint] { continue; }   // This spot is already connected from somewhere, let's backtrack

			*self.maze.get_mut(p.y as uint).get_mut(p.x as uint) = true;        // Mark this spot as visited
			*self.maze.get_mut(wall.y as uint).get_mut(wall.x as uint) = true;  // Also remove the wall between this spot and the last one

			// `nexts` vector holds all the positions that the algorithm might go to next
			// (without stepping outside the maze) along with positions that connect them the current one
			let mut nexts: Vec<(Point, Point)> = Vec::new();
			for d in dirs.iter() {
				let nwall = p + *d;   // points to the wall between this spot and the next one
				let np = nwall + *d;  // points to the next spot
				if np.within(1, 1, self.width as int - 2, self.height as int -2)
				{
					nexts.push((np, nwall));
				}
			}

			task_rng().shuffle(nexts.as_mut_slice());  // Shuffles the vector of next possible spots, this is where the randomization happens
			stack.push_all(nexts.as_slice());          // Push all the `nexts` (now shuffled) onto the DFS stack
		}
	}

	fn make_pbm(&self) -> String {
		// Generates output PBM format, one pixels for each spot of the maze grid

		let mut pbm = format!("P1\n# Generated by maze generator from rust-homework\n{:u} {:u}\n", self.width, self.height);

		for col in self.maze.iter() {
			for b in col.iter() {
				pbm.push_str(if *b { "0 " } else { "1 "});
			}
			pbm.push_str("\n");
		}

		pbm
	}
}


fn main() {
	let mut maze = Maze::new(81, 41);
	maze.randomize(0, 0);
	print!("{:s}", maze.make_pbm());
}