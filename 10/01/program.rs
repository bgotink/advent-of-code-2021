use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut score = 0;
    
    for line in lines {
      let mut stack: VecDeque<char> = VecDeque::new();
      
      for c in line.unwrap().chars() {
        match c {
          '[' | '(' | '{' | '<' => {
            stack.push_front(c);
          },
          ']' => {
            if stack.pop_front().unwrap() != '[' {
              score += 57;
            }
          },
          ')' => {
            if stack.pop_front().unwrap() != '(' {
              score += 3;
            }
          },
          '}' => {
            if stack.pop_front().unwrap() != '{' {
              score += 1197;
            }
          },
          '>' => {
            if stack.pop_front().unwrap() != '<' {
              score += 25137;
            }
          },
          _ => {}
        }
      }
    }
    
    println!("{}", score);
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