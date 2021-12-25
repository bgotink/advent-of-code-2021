use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

#[derive(Clone,Copy,Hash,PartialEq,Eq)]
enum State {
  Empty,
  South,
  East,
}

impl State {
  fn parse(c: char) -> State {
    match c {
      '>' => State::East,
      'v' => State::South,
      '.' => State::Empty,
      _ => panic!("unexpected character '{}'", c),
    }
  }

  fn to_char(&self) -> char {
    match self {
      State::East => '>',
      State::South => 'v',
      State::Empty => '.',
    }
  }
}

impl std::fmt::Display for State {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      State::East => write!(f, ">"),
      State::South => write!(f, "v"),
      State::Empty => write!(f, "."),
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut board: Vec<Vec<State>> = lines.map(|line| line.unwrap().chars().map(|c| State::parse(c)).collect()).collect();

    let max_y = board.len();
    let max_x = board.iter().next().unwrap().len();

    // println!("Initial state:");
    // for line in board.iter() {
    //   println!("  {}", line.iter().map(|s| s.to_char()).collect::<String>());
    // }
    // println!("");

    let mut changed = true;
    let mut nb_iterations = 0;
    while changed {
      changed = false;
      nb_iterations += 1;

      let mut can_move: HashSet<(usize, usize)> = HashSet::new();

      // First West to East
      for y in 0..max_y {
        for x in 0..max_x {
          if board[y][x] == State::East && board[y][(x + 1) % max_x] == State::Empty {
            can_move.insert((x, y));
          }
        }
      }
      for y in 0..max_y {
        for x in 0..max_x {
          if can_move.contains(&(x, y)) {
            changed = true;
            board[y][x] = State::Empty;
            board[y][(x + 1) % max_x] = State::East;
          }
        }
      }

      can_move.clear();

      // Then North to South
      for y in 0..max_y {
        for x in 0..max_x {
          if board[y][x] == State::South && board[(y + 1) % max_y][x] == State::Empty {
            can_move.insert((x, y));
          }
        }
      }
      for y in 0..max_y {
        for x in 0..max_x {
          if can_move.contains(&(x, y)) {
            board[(y + 1) % max_y][x] = State::South;
            board[y][x] = State::Empty;
            changed = true;
          }
        }
      }

      // println!("Iteration {}:", nb_iterations);
      // for line in board.iter() {
      //   println!("  {}", line.iter().map(|s| s.to_char()).collect::<String>());
      // }
      // println!("");
    }

    println!("Stopped moving after {} iterations", nb_iterations);
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