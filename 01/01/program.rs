use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  let mut up = 0;
  let mut previous = i32::MAX;

  if let Ok(lines) = read_lines(&args[1]) {
    for line in lines {
      let current = line.unwrap().parse::<i32>().unwrap();
      
      if current > previous {
        up += 1;
      }

      previous = current;
    }
  } else {
    panic!("Failed to read file");
  }

  print!("{}\n", up);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}