use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::convert::TryInto;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    // We'll use a list of numbers to count the amount of times 1 is present more often than 0
    let mut counters: Vec<i32> = Vec::new();

    for line in lines {
      let l = line.unwrap();

      if counters.len() == 0 {
        counters = vec![0; l.len().try_into().unwrap()];
      }

      for (idx, c) in l.chars().enumerate() {
        match c {
          '1' => counters[idx] += 1,
          '0' => counters[idx] -= 1,
          _ => panic!("Unexpected {} in \"{}\"", c, l),
        }
      }
    }
      
    let mut epsilon = 0;
    let mut gamma = 0;

    for c in counters {
      let gampart = if c > 0 { 1 } else { 0 };
      let epspart = 1 - gampart;

      epsilon = (2 * epsilon) + epspart;
      gamma = (2 * gamma) + gampart;
    }

    println!("{}", epsilon * gamma);
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