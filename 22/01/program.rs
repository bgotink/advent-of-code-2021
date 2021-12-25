use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

#[derive(Clone,Copy,Hash,PartialEq,Eq)]
enum State {
  Off,
  On,
}

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
struct Point {
  x: i32,
  y: i32,
  z: i32,
}

impl std::fmt::Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}

fn min_max(one: &Point, two: &Point) -> (Point, Point) {
  let Point { x: one_x, y: one_y, z: one_z } = one;
  let Point { x: two_x, y: two_y, z: two_z } = two;

  (Point {
    x: cmp::min(*one_x, *two_x),
    y: cmp::min(*one_y, *two_y),
    z: cmp::min(*one_z, *two_z),
  },
  Point {
    x: cmp::max(*one_x, *two_x),
    y: cmp::max(*one_y, *two_y),
    z: cmp::max(*one_z, *two_z),
  })
}

trait Cuboid {
  fn min(&self) -> Point;
  fn max(&self) -> Point;

  fn size(&self) -> u64 {
    let min = self.min();
    let max = self.max();

    let dx = (max.x - min.x) as u64;
    let dy = (max.y - min.y) as u64;
    let dz = (max.z - min.z) as u64;

    dx * dy * dz
  }

  fn corners(&self) -> [Point; 8] {
    let Point { x: min_x, y: min_y, z: min_z } = self.min().clone();
    let Point { x: max_x, y: max_y, z: max_z } = self.max().clone();

    [
      Point { x: min_x, y: min_y, z: min_z },
      Point { x: max_x, y: min_y, z: min_z },
      Point { x: min_x, y: max_y, z: min_z },
      Point { x: max_x, y: max_y, z: min_z },
      Point { x: min_x, y: min_y, z: max_z },
      Point { x: max_x, y: min_y, z: max_z },
      Point { x: min_x, y: max_y, z: max_z },
      Point { x: max_x, y: max_y, z: max_z },
    ]
  }
  
  fn contains_point(&self, point: &Point) -> bool {
    if point.x < self.min().x || point.x >= self.max().x {
      false
    } else if point.y < self.min().y || point.y >= self.max().y {
      false
    } else if point.z < self.min().z || point.z >= self.max().z {
      false
    } else {
      true
    }
  }
  
  fn contains_cuboid(&self, other: &dyn Cuboid) -> bool {
    if !self.contains_point(&other.min()) {
      false
    } else {
      let max = self.max();
      let othermax = other.max();

      othermax.x <= max.x && othermax.y <= max.y && othermax.z <= max.z
    }
  }

  fn overlaps(&self, other: &dyn Cuboid) -> bool {
    if self.min().x >= other.max().x || other.min().x >= self.max().x {
      false
    } else if self.min().y >= other.max().y || other.min().y >= self.max().y {
      false
    } else if self.min().z >= other.max().x || other.min().x >= self.max().x {
      false
    } else {
      true
    }
  }

  fn intersection(&self, other: &dyn Cuboid) -> Option<SimpleCuboid> {
    let (_, min) = min_max(&self.min(), &other.min());
    let (max, _) = min_max(&self.max(), &other.max());

    if min.x < max.x && min.y < max.y && min.z < max.z {
      Some(SimpleCuboid{ min: min, max: max })
    } else {
      None
    }
  }
}

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
struct SimpleCuboid {
  min: Point,
  max: Point,
}

impl Cuboid for SimpleCuboid {
  fn min(&self) -> Point {
    self.min
  }

  fn max(&self) -> Point {
    self.max
  }
}

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
struct StateCuboid {
  min: Point,
  max: Point,

  state: State,
}

impl Cuboid for StateCuboid {
  fn min(&self) -> Point {
    self.min
  }

  fn max(&self) -> Point {
    self.max
  }
}

fn parse_min_max(str: &str) -> (i32, i32) {
  let mut parts = str[2..].split("..");

  let min = parts.next().unwrap().parse::<i32>().unwrap();
  let max = parts.next().unwrap().parse::<i32>().unwrap();

  if parts.next() != None {
    panic!("Invalid line {}", str);
  }

  assert!(min < max);

  (min, max)
}

impl StateCuboid {
  fn parse(line: String) -> StateCuboid {
    if let Some(space_idx) = line.find(' ') {
      let state = match &line[0..space_idx] {
        "on" => State::On,
        _ => State::Off,
      };

      let coordinates = line[(space_idx + 1)..].split(',').collect::<Vec<_>>();
      assert_eq!(coordinates.len(), 3);

      let (min_x, max_x) = parse_min_max(coordinates[0]);
      let (min_y, max_y) = parse_min_max(coordinates[1]);
      let (min_z, max_z) = parse_min_max(coordinates[2]);

      StateCuboid {
        state: state,
        min: Point {
          x: min_x,
          y: min_y,
          z: min_z,
        },
        max: Point {
          x: max_x + 1,
          y: max_y + 1,
          z: max_z + 1,
        },
      }
    } else {
      panic!("Invalid line {}", line);
    }
  }
}

enum CuboidTreeState {
  Leaf(State),
  Split(Box<CuboidTree>, Box<CuboidTree>),
}

/// A binary tree of cuboids
///
/// A CuboidTree node has either a state (CuboidTreeState::Leaf) or it has two children
/// which split the cuboid along the X, Y, or Z axis.
struct CuboidTree {
  min: Point,
  max: Point,

  state: CuboidTreeState,
}

impl Cuboid for CuboidTree {
  fn min(&self) -> Point {
    self.min
  }

  fn max(&self) -> Point {
    self.max
  }
}

impl CuboidTree {
  fn size_on(&self) -> u64 {
    match self.state {
      CuboidTreeState::Leaf(state) => if state == State::On { self.size() } else { 0 },
      CuboidTreeState::Split(ref left, ref right) => left.size_on() + right.size_on(),
    }
  }

  fn apply(&mut self, state: &StateCuboid) -> () {
    if state.contains_cuboid(self) {
      // entire self is contained in the state Cuboid, so...
      self.state = CuboidTreeState::Leaf(state.state);
      return;
    }

    if let Some(intersection) = self.intersection(state) {
      match self.state {
        CuboidTreeState::Split(ref mut left, ref mut right) => {
          left.apply(state);
          right.apply(state);
        },
        CuboidTreeState::Leaf(original_state) => {
          if state.state == original_state {
            // We already have the right state, so...
            return;
          }

          // Find a good point to split the tree further

          let intersection_min = intersection.min();
          let intersection_max = intersection.max();

          let min = self.min();
          let max = self.max();

          let new_min: Point;
          let new_max: Point;

          if intersection_min.x > min.x || intersection_max.x < max.x {
            let cutoff = if intersection_min.x - min.x > max.x - intersection_max.x {
              intersection_min.x
            } else {
              intersection_max.x
            };

            new_min = Point { x: cutoff, y: min.y, z: min.z };
            new_max = Point { x: cutoff, y: max.y, z: max.z };
          } else if intersection_min.y > min.y || intersection_max.y < max.y {
            let cutoff = if intersection_min.y - min.y > max.y - intersection_max.y {
              intersection_min.y
            } else {
              intersection_max.y
            };

            new_min = Point { x: min.x, y: cutoff, z: min.z };
            new_max = Point { x: max.x, y: cutoff, z: max.z };
          } else if intersection_min.z > min.z || intersection_max.z < max.z {
            let cutoff = if intersection_min.z - min.z > max.z - intersection_max.z {
              intersection_min.z
            } else {
              intersection_max.z
            };

            new_min = Point { x: min.x, y: min.y, z: cutoff };
            new_max = Point { x: max.x, y: max.y, z: cutoff };
          } else {
            panic!("couldn't find point to cut");
          }

          let mut left = CuboidTree {
            min: min,
            max: new_max,
  
            state: CuboidTreeState::Leaf(original_state),
          };
          left.apply(state);

          let mut right = CuboidTree {
            min: new_min,
            max: max,
  
            state: CuboidTreeState::Leaf(original_state),
          };
          right.apply(state);
  
          self.state = CuboidTreeState::Split(
            Box::new(left),
            Box::new(right),
          );
        }
      }
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut root = CuboidTree {
      min: Point { x: -50, y: -50, z: -50 },
      max: Point { x:  51, y:  51, z:  51 },
      state: CuboidTreeState::Leaf(State::Off),
    };

    for line in lines {
      root.apply(&StateCuboid::parse(line.unwrap()));
    }

    println!("on: {}", root.size_on());
  } else {
    panic!("Failed to read file");
  }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}