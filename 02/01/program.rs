use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::error::Error;

enum Instruction {
  Forward(i32),
  Up(i32),
  Down(i32),
}

impl FromStr for Instruction {
  type Err = Box<dyn Error>;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.starts_with("forward ") {
      Ok(Instruction::Forward(s.strip_prefix("forward ").unwrap().parse::<i32>()?))
    } else if s.starts_with("down ") {
      Ok(Instruction::Down(s.strip_prefix("down ").unwrap().parse::<i32>()?))
    } else if s.starts_with("up ") {
      Ok(Instruction::Up(s.strip_prefix("up ").unwrap().parse::<i32>()?))
    } else {
      return Result::Err(format!("Unexpected instruction \"{}\"", s).into());
    }
  }
}

struct Position {
  position: i32,
  depth: i32,
}

impl Position {
  fn apply(&self, instr: &Instruction) -> Self {
    let Self {mut position, mut depth} = self;

    match instr {
      Instruction::Up(v) => depth -= v,
      Instruction::Down(v) => depth += v,
      Instruction::Forward(v) => position += v,
    };

    return Self {position, depth};
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  let mut position = Position { position: 0, depth: 0 };

  if let Ok(lines) = read_lines(&args[1]) {
    for line in lines {
      let current = line.unwrap().parse::<Instruction>().unwrap();
      
      position = position.apply(&current);
    }
  } else {
    panic!("Failed to read file");
  }

  print!("{}\n", position.position * position.depth);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}