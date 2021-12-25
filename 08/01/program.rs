use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_line(line: &str) -> (Vec<String>, Vec<String>) {
  let parts: Vec<_> = line.split(" | ").collect();

  if parts.len() != 2 {
    panic!("invalid line");
  }

  (
    parts[0].split_whitespace().map(|x| String::from(x)).collect(),
    parts[1].split_whitespace().map(|x| String::from(x)).collect()
  )
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut count = 0;

    for line in lines {
      let (_ , output) = parse_line(&line.unwrap());

      for n in output {
        match n.len() {
          2 | 3 | 4 | 7 => count += 1,
          _ => ()
        }
      }
    }
    
    println!("{}", count);
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