use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;
use std::collections::VecDeque;

struct Arena {
  nodes: Vec<Node>,
}

impl Arena {
  fn new() -> Self {
    Self { nodes: Vec::from([Node::Empty]) }
  }

  fn leaf(&mut self, val: i32) -> usize {
    let idx = self.nodes.len();

    self.nodes.push(Node::Leaf(val));

    idx
  }

  fn tree(&mut self) -> usize {
    let idx = self.nodes.len();

    self.nodes.push(Node::Tree { one: 0, zero: 0, count: 0 });

    idx
  }

  fn get(&self, idx: usize) -> &Node {
    &self.nodes[idx]
  }

  fn append(&mut self, root: usize, line: &str) -> Result<(), Box<dyn Error>> {
    let mut val: i32 = 0;

    for c in line.chars() {
      val = val * 2;

      if c == '1' {
        val += 1;
      }
    }

    let mut chars = line.chars().collect::<VecDeque<char>>();
    self._append(root, &mut chars, val)
  }

  fn _append(&mut self, tree: usize, line: &mut VecDeque<char>, val: i32) -> Result<(), Box<dyn Error>> {
    if let Node::Tree { mut one, mut zero, mut count } = self.nodes[tree] {
      count += 1;

      match line.pop_front() {
        Some('1') => {
          if line.len() == 0 {
            one = self.leaf(val);
            self.nodes[tree] = Node::Tree { one, zero, count };
            Result::Ok(())
          } else {
            if let Node::Empty = self.nodes[one] {
              one = self.tree();
            }

            self.nodes[tree] = Node::Tree { one, zero, count };
            self._append(one, line, val)
          }
        },
        Some('0') => {
          if line.len() == 0 {
            zero = self.leaf(val);
            self.nodes[tree] = Node::Tree { one, zero, count };
            Result::Ok(())
          } else {
            if let Node::Empty = self.nodes[zero] {
              zero = self.tree();
            }

            self.nodes[tree] = Node::Tree { one, zero, count };
            self._append(zero, line, val)
          }
        },
        Some(v) => Err(format!("unexpected value in binary: \"{}\"", v).into()),
        _ => Err("unexpected short".into()),
      }
    } else {
      Err("unexpected leaf".into())
    }
  }

  fn get_count(&self, idx: usize) -> i32 {
    match self.nodes[idx] {
      Node::Tree { count, .. } => count,
      Node::Leaf(_) => 1,
      _ => 0
    }
  }
}

enum Node {
  Leaf(i32),
  Empty,

  Tree { one: usize, zero: usize, count: i32 }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut arena = Arena::new();
    let root = arena.tree();

    for line in lines {
      arena.append(root, &line.unwrap()).unwrap();
    }

    let mut o2generator_index = root;
    while let Node::Tree { one, zero, .. } = arena.get(o2generator_index) {
      let nbone = arena.get_count(*one);
      let nbzero = arena.get_count(*zero);

      if nbone >= nbzero {
        o2generator_index = *one;
      } else {
        o2generator_index = *zero;
      }
    }

    let mut co2scrubber_index = root;
    while let Node::Tree { one, zero, count } = arena.get(co2scrubber_index) {
      if *count == 1 {
        co2scrubber_index = if arena.get_count(*one) == 1 { *one } else { *zero };
      } else {
        let nbone = arena.get_count(*one);
        let nbzero = arena.get_count(*zero);
  
        if nbone == 0 || nbzero > 0 && nbone >= nbzero {
          co2scrubber_index = *zero;
        } else {
          co2scrubber_index = *one;
        }
      }
    }

    // println!("found {} and {}", o2generator_index, co2scrubber_index);

    match arena.get(o2generator_index) {
      Node::Leaf(o2generator) => {
        match arena.get(co2scrubber_index) {
          Node::Leaf(co2scrubber) => println!("{}", o2generator * co2scrubber),
          _ => panic!("expected to find co2scrubber")
        }
      },
      _ => panic!("expected to find o2generator")
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