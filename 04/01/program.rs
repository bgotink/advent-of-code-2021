use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;
use std::collections::HashSet;
use std::iter::FromIterator;


fn parse_line(line: &str) -> Vec<i32> {
  line.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect()
}

struct Board {
  open_numbers: HashSet<i32>,

  lines: Vec<HashSet<i32>>,
}

impl Board {
  fn new(parsed_lines: &Vec<Vec<i32>>) -> Result<Board, Box<dyn Error>> {
    if parsed_lines.len() != 5 {
      return Err("expected 5 lines".into())
    }
    
    let mut bingo_lines = parsed_lines.iter().map(|l| HashSet::from_iter(l.iter().cloned())).collect::<Vec<HashSet<i32>>>();
    bingo_lines.append(
      &mut (0..4).map(|i| parsed_lines.iter().map(|line| line[i]).collect::<HashSet<i32>>()).collect::<Vec<HashSet<i32>>>()
    );

    let mut all: HashSet<i32> = HashSet::new();

    for line in parsed_lines {
      for nb in line {
        all.insert(*nb);
      }
    }

    Ok(Board { open_numbers: all, lines: bingo_lines })
  }

  fn open_value(&self) -> i32 {
    self.open_numbers.iter().sum()
  }

  fn remove(&mut self, value: &i32) -> bool {
    if self.open_numbers.remove(value) {
      for line in &mut self.lines {
        (*line).remove(value);

        if line.len() == 0 {
          return true
        }
      }
    }

    false
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    // The picked numbers
    let mut numbers: Vec<i32> = Vec::new();
    // The boards
    let mut boards: Vec<Board> = Vec::new();

    let mut tmp: Vec<Vec<i32>> = Vec::new();
    for line in lines {
      let l = line.unwrap();

      if l.len() == 0 {
        continue;
      } else if numbers.len() == 0 {
        numbers = l.split(',').map(|v| v.parse::<i32>().unwrap()).collect()
      } else {
        tmp.push(parse_line(&l));

        if tmp.len() == 5 {
          if let Ok(board) = Board::new(&tmp) {
            boards.push(board);
          } else {
            panic!("failed to parse board");
          }
          tmp = Vec::new();
        }
      }
    }

    if tmp.len() != 0 {
      panic!("unexpected extra line(s)");
    }

    for nb in numbers {
      for board in &mut boards {
        if board.remove(&nb) {
          println!("{}", nb * board.open_value());
          return;
        }
      }
    }

    panic!("No board matched");
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