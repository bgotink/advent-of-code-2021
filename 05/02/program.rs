use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;
use std::collections::HashSet;
use std::ops;
use std::convert::TryInto;

#[derive(Clone, Copy)]
struct Vector {
  x: i32,
  y: i32,
}

struct VectorIterator {
  unit: Vector,
  current: usize,
  max: usize,
}

impl Iterator for VectorIterator {
  type Item = Vector;

  fn next(&mut self) -> Option<Self::Item> {
    if self.current > self.max {
      None
    } else {
      let current = self.current;
      self.current += 1;

      Some(self.unit * current)
    }
  }
}

impl Vector {
  fn is_horizontal(&self) -> bool {
    self.y == 0
  }

  fn is_vertical(&self) -> bool {
    self.x == 0
  }

  fn len(&self) -> usize {
    if self.x == 0 {
      self.y.abs().try_into().unwrap()
    } else if self.y == 0 {
      self.x.abs().try_into().unwrap()
    } else if self.x.abs() == self.y.abs() {
      self.y.abs().try_into().unwrap()
    } else {
      panic!("only horizontal, vertical, and diagonal lines are supported");
    }
  }

  fn iter(self) -> VectorIterator {
    let len = self.len();
    if len == 0 {
      VectorIterator {current: 0, max: 0, unit: self}
    } else {
      VectorIterator { current: 0, max: len, unit: self / len }
    }
  }
}

impl ops::Mul<i32> for Vector {
  type Output = Vector;

  fn mul(self, rhs: i32) -> Vector {
    Vector { x: self.x * rhs, y: self.y * rhs }
  }
}

impl ops::Mul<usize> for Vector {
  type Output = Vector;

  fn mul(self, rhs: usize) -> Vector {
    let r: i32 = rhs.try_into().unwrap();
    Vector { x: self.x * r, y: self.y * r }
  }
}

impl ops::Div<i32> for Vector {
  type Output = Vector;

  fn div(self, rhs: i32) -> Vector {
    Vector { x: self.x / rhs, y: self.y / rhs }
  }
}

impl ops::Div<usize> for Vector {
  type Output = Vector;

  fn div(self, rhs: usize) -> Vector {
    let r: i32 = rhs.try_into().unwrap();
    Vector { x: self.x / r, y: self.y / r }
  }
}

impl ops::Add<Vector> for Vector {
  type Output = Vector;

  fn add(self, rhs: Vector) -> Vector {
    Vector { x: self.x + rhs.x, y: self.y + rhs.y }
  }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
  x: i32,
  y: i32,
}

impl Point {
  fn parse(raw: &str) -> Result<Point, Box<dyn Error>> {
    let parts = raw.split(',').collect::<Vec<&str>>();

    if parts.len() != 2 {
      return Err(format!("invalid point: \"{}\"", raw).into());
    }

    let x = parts[0].parse::<i32>()?;
    let y = parts[1].parse::<i32>()?;

    Ok(Point { x, y })
  }

}

impl ops::Add<Vector> for Point {
  type Output = Point;

  fn add(self, rhs: Vector) -> Point {
    Point { x: self.x + rhs.x, y: self.y + rhs.y }
  }
}

impl ops::Sub<Point> for Point {
  type Output = Vector;

  fn sub(self, rhs: Point) -> Vector {
    Vector { x: self.x - rhs.x, y: self.y - rhs.y }
  }
}

fn parse_line(line: &str) -> Result<(Point, Point), Box<dyn Error>> {
  let parts = line.split(" -> ").collect::<Vec<&str>>();

  if parts.len() != 2 {
    return Err(format!("invalid line: \"{}\"", line).into());
  }

  Ok((Point::parse(parts[0])?, Point::parse(parts[1])?))
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut active: HashSet<Point> = HashSet::new();
    let mut doubles: HashSet<Point> = HashSet::new();
    
    for line in lines {
      let (start, end) = parse_line(&line.unwrap()).unwrap();

      let diff = end - start;

      // println!("{},{} -> {},{}", start.x, start.y, end.x, end.y);
      // println!("{},{}", diff.x, diff.y);

      // let all = diff.iter().collect::<Vec<Vector>>();

      // if diff.len() + 1 != all.len() {
      //   println!("{},{} -> {},{}", start.x, start.y, end.x, end.y);
      //   println!("{},{}", diff.x, diff.y);
      // }

      for d in diff.iter() {
        let current = start + d;

        if active.contains(&current) {
          doubles.insert(current);
        } else {
          active.insert(current);
        }
      }
    }

    println!("{}", doubles.len());
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