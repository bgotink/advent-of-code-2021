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
    let mut scores: Vec<u64> = Vec::new();
    
    'outer: for line in lines {
      let mut stack: VecDeque<char> = VecDeque::new();
      
      for c in line.unwrap().chars() {
        match c {
          ']' | ')' | '}' | '>' => {
            if stack.pop_front().unwrap() != c {
              continue 'outer;
            }
          },
          '[' => {
            stack.push_front(']');
          },
          '(' => {
            stack.push_front(')');
          },
          '{' => {
            stack.push_front('}');
          },
          '<' => {
            stack.push_front('>');
          },
          _ => {}
        }
      }

      let mut line_score: u64 = 0;

      for c in stack.iter() {
        line_score = 5 * line_score + match c {
          ')' => 1,
          ']' => 2,
          '}' => 3,
          '>' => 4,
          _ => unreachable!(),
        }
      }

      if line_score != 0 {
        println!("score: {} for ending {}", line_score, stack.iter().collect::<String>());
        scores.push(line_score);
      }
    }

    if scores.len() % 2 == 0 {
      panic!("expected uneven number of incomplete lines but got {}", scores.len());
    }
    
    scores.sort_unstable();
    println!("\n{}", scores[scores.len() / 2]);
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