use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

#[derive(PartialEq,Eq,Hash,Debug,Copy,Clone)]
struct Point {
  x: u32,
  y: u32,
}

fn flip(offset: u32, val: u32) -> u32 {
  (2 * offset).checked_sub(val).unwrap()
}

impl Point {
  fn flip_x(&self, offset: u32) -> Point {
    if self.x > offset {
      Point { x: flip(offset, self.x), y: self.y }
    } else {
      *self
    }
  }

  fn flip_y(&self, offset: u32) -> Point {
    if self.y > offset {
      Point { x: self.x, y: flip(offset, self.y) }
    } else {
      *self
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut dots: HashSet<Point> = HashSet::new();

    for line in lines {
      let l = line.unwrap();
      
      if ! l.starts_with("fold along") {
        if l.len() > 0 {
          let parts = l.split(',').collect::<Vec<_>>();

          let x = parts[0].parse::<u32>().unwrap();
          let y = parts[1].parse::<u32>().unwrap();

          dots.insert(Point { x: x, y: y });
        }
      } else {
        let offset: u32;
        {
          let offset_str = &l[13..];

          offset = offset_str.parse::<u32>().unwrap();
        }

        let mut new_dots: HashSet<Point> = HashSet::new();

        match l.chars().take(12).last().unwrap() {
          'x' => {
            for d in dots {
              new_dots.insert(d.flip_x(offset));
            }
          },
          'y' => {
            for d in dots {
              new_dots.insert(d.flip_y(offset));
            }
          },
          c => panic!("unexpected axis \"{}\"", c),
        }

        dots = new_dots;

        println!("{}", dots.len());
        return;
      }
    }
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