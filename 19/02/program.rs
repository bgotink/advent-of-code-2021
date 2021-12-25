use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::ops;

#[derive(PartialEq,Eq,Hash,Copy,Clone)]
struct Point {
  x: i32,
  y: i32,
  z: i32,
}

impl Point {
  const ORIGIN: Point = Point { x: 0, y: 0, z: 0 };

  fn parse(s: String) -> Point {
    let parts: Vec<_> = s.split(',').collect();

    if parts.len() != 3 {
      panic!("Invalid point: \"{}\"", s);
    }

    let x = parts[0].parse::<i32>().unwrap();
    let y = parts[1].parse::<i32>().unwrap();
    let z = parts[2].parse::<i32>().unwrap();

    Point { x: x, y: y, z: z }
  }

  fn as_vector(&self) -> Vector {
    Vector { x: self.x, y: self.y, z: self.z }
  }

  fn manhattan_distance(&self, other: &Point) -> i32 {
    (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
  }
}

#[derive(PartialEq,Eq,Hash,Copy,Clone)]
struct Vector {
  x: i32,
  y: i32,
  z: i32,
}

impl Vector {
  fn len(&self) -> f32 {
    (self.dot(self) as f32).sqrt()
  }

  fn dot(&self, rhs: &Vector) -> i32 {
    self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
  }

  fn cross(&self, rhs: &Vector) -> Vector {
    Vector {
      x: self.y * rhs.z - self.z * rhs.y,
      y: self.z * rhs.x - self.x * rhs.z,
      z: self.x * rhs.y - self.y * rhs.x,
    }
  }
}

impl std::fmt::Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Point ({}, {}, {})", self.x, self.y, self.z)
  }
}

impl std::fmt::Display for Vector {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Vector ({}, {}, {})", self.x, self.y, self.z)
  }
}

impl ops::Add<Vector> for &Point {
  type Output = Point;

  fn add(self, rhs: Vector) -> Point {
    Point {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl ops::Add<&Vector> for &Point {
  type Output = Point;

  fn add(self, rhs: &Vector) -> Point {
    Point {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl ops::Add<Vector> for Point {
  type Output = Point;

  fn add(self, rhs: Vector) -> Point {
    Point {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl ops::Add<&Vector> for &Vector {
  type Output = Vector;

  fn add(self, rhs: &Vector) -> Vector {
    Vector {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl ops::Add<Vector> for Vector {
  type Output = Vector;

  fn add(self, rhs: Vector) -> Vector {
    Vector {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl ops::Sub<&Vector> for &Point {
  type Output = Point;

  fn sub(self, rhs: &Vector) -> Point {
    Point {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl ops::Sub<Vector> for Point {
  type Output = Point;

  fn sub(self, rhs: Vector) -> Point {
    Point {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl ops::Sub<&Vector> for &Vector {
  type Output = Vector;

  fn sub(self, rhs: &Vector) -> Vector {
    Vector {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl ops::Sub<Vector> for Vector {
  type Output = Vector;

  fn sub(self, rhs: Vector) -> Vector {
    Vector {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl ops::Sub<&Point> for &Point {
  type Output = Vector;

  fn sub(self, rhs: &Point) -> Vector {
    Vector {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl ops::Sub<Point> for Point {
  type Output = Vector;

  fn sub(self, rhs: Point) -> Vector {
    Vector {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl ops::Mul<i32> for &Vector {
  type Output = Vector;

  fn mul(self, rhs: i32) -> Vector {
    Vector {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    }
  }
}

impl ops::Mul<i32> for Vector {
  type Output = Vector;

  fn mul(self, rhs: i32) -> Vector {
    Vector {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    }
  }
}

impl ops::Div<i32> for &Vector {
  type Output = Vector;

  fn div(self, rhs: i32) -> Vector {
    Vector {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
    }
  }
}

impl ops::Div<i32> for Vector {
  type Output = Vector;

  fn div(self, rhs: i32) -> Vector {
    Vector {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
    }
  }
}

#[derive(PartialEq,Eq,Hash,Copy,Clone)]
struct Orientation {
  positive_x: Vector,
  positive_y: Vector,
  positive_z: Vector,
}

impl Orientation {
  fn create(x: Vector, y: Vector) -> Orientation {
    let z = x.cross(&y);
    Orientation { positive_x: x, positive_y: y, positive_z: z }
  }

  fn all() -> HashSet<Orientation> {
    let mut all = HashSet::new();

    let real_x = Vector { x: 1, y: 0, z: 0 };
    let real_y = Vector { x: 0, y: 1, z: 0 };
    let real_z = Vector { x: 0, y: 0, z: 1 };

    for x_mult in [1, -1] {
      for y_mult in [1, -1] {
        all.insert(Orientation::create(&real_x * x_mult, &real_y * y_mult));
        all.insert(Orientation::create(&real_x * x_mult, &real_z * y_mult));
        all.insert(Orientation::create(&real_y * x_mult, &real_x * y_mult));
        all.insert(Orientation::create(&real_y * x_mult, &real_z * y_mult));
        all.insert(Orientation::create(&real_z * x_mult, &real_x * y_mult));
        all.insert(Orientation::create(&real_z * x_mult, &real_y * y_mult));
      }
    }

    assert_eq!(all.len(), 24);

    all
  }

  fn map(&self, absolute_vector: &Vector) -> Vector {
    &self.positive_x * absolute_vector.x + &self.positive_y * absolute_vector.y + &self.positive_z * absolute_vector.z
  }

  fn unmap(&self, vector: Vector) -> Vector {
    Vector {
      x: vector.dot(&self.positive_x),
      y: vector.dot(&self.positive_y),
      z: vector.dot(&self.positive_z),
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut measurements: Vec<Option<Vec<Point>>> = Vec::new();
    let mut current_measurement: Option<Vec<Point>> = None;

    for l in lines {
      let line = l.unwrap();

      if line.len() == 0 {
        match current_measurement {
          Some(points) => {
            measurements.push(Some(points));
            current_measurement = None;
          },
          None => panic!("Expected points"),
        };
      } else {
        match current_measurement {
          Some(ref mut points) => points.push(Point::parse(line)),
          None => {
            current_measurement = Some(Vec::new());
          },
        };
      }
    }

    if let Some(points) = current_measurement {
      measurements.push(Some(points));
    }

    let orientations = Orientation::all();
    let mut absolute_points: HashSet<Point> = HashSet::new();
    let mut scanner_locations: Vec<Point> = Vec::new();

    {
      for point in std::mem::replace(&mut measurements[0], None).unwrap() {
        absolute_points.insert(point);
      }
    }

    let mut num_measurements = measurements.len() - 1;
    while num_measurements > 0 {
      let mut found_match: Option<(usize, Point, Point, &Orientation)> = None;

      'outer: for (i, option_measurement) in measurements.iter().enumerate() {
        if let Some(ref measurement) = option_measurement {
          for origin in measurement.iter() {
            for absolute_origin in absolute_points.iter() {
              for orientation in orientations.iter() {
                let mut num_matches = 0;
  
                for point in measurement.iter() {
                  let absolute_point = absolute_origin + orientation.unmap(point - origin);
  
                  if absolute_points.contains(&absolute_point) {
                    num_matches += 1;
                  }
                }
  
                if num_matches >= 12 {
                  found_match = Some((i, origin.clone(), absolute_origin.clone(), orientation));
                  break 'outer;
                }
              }
            }
          }
        }
      }

      match found_match {
        Some((i, origin, absolute_origin, orientation)) => {
          println!("Mapping measurement {} point {} to {}", i, origin, absolute_origin);

          for point in std::mem::replace(&mut measurements[i], None).unwrap() {
            // println!(" {} -> {}", point, absolute_origin + orientation.unmap(point - origin));
            absolute_points.insert(absolute_origin + orientation.unmap(point - origin));
          }

          scanner_locations.push(
            absolute_origin + orientation.unmap(Point::ORIGIN - origin)
          );

          num_measurements -= 1;
        },
        None => {
          println!("Failed to place {} measurements", num_measurements);

          for (i, meas) in measurements.into_iter().enumerate() {
            println!("- was {} placed? {}", i, meas.is_none());
          }

          panic!();
        },
      };
    }

    let mut max_distance = 0_i32;
    for a in scanner_locations.iter() {
      for b in scanner_locations.iter() {
        let distance = a.manhattan_distance(b);

        if distance > max_distance {
          max_distance = distance;
        }
      }
    }

    println!("\nMax scanner distance: {}", max_distance);
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