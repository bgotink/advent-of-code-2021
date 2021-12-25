use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;
use std::collections::VecDeque;

fn parse_line(line: &str) -> Result<VecDeque<u64>, Box<dyn Error>> {
  let mut position: Vec<u64> = vec![0; 9];

  for part in line.split(',') {
    position[part.parse::<usize>()?] += 1;
  }

  Ok(VecDeque::from(position))
}

fn step(state: &mut VecDeque<u64>) {
  let doubling = state.pop_front().unwrap();

  state.push_back(doubling);
  state[6] += doubling;
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    for line in lines {
      let mut state = parse_line(&line.unwrap()).unwrap();

      for _i in 0..256 {
        step(&mut state);
      }

      println!("{}", state.iter().sum::<u64>());
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