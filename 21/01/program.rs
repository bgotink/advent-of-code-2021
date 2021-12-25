use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Die {
  val: u32,
  max: u32,

  number_of_casts: u32,
}

impl Die {
  fn new() -> Die {
    Die { val: 1, max: 100, number_of_casts: 0 }
  }

  fn cast(&mut self) -> u32 {
    let val = self.val % self.max;
    
    self.val += 1;
    self.number_of_casts += 1;
    
    val
  }
}

fn has_winner(scores: &Vec<u32>) -> bool {
  scores.iter().any(|v| { *v >= 1000 })
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut die = Die::new();
    let mut positions: Vec<u32> = lines.map(|line| {
      line.unwrap().chars().last().unwrap().to_digit(10).unwrap()
    }).collect();
    let mut scores: Vec<u32> = positions.iter().map(|_| 0).collect();

    println!("Starting positions:");
    for (i, position) in positions.iter().enumerate() {
      println!("- {}: {}", i, position);
    }

    while !has_winner(&scores) {
      for (i, position) in positions.iter_mut().enumerate() {
        let cast_one = die.cast();
        let cast_two = die.cast();
        let cast_three = die.cast();

        // *position += die.cast() + die.cast() + die.cast();
        *position += cast_one + cast_two + cast_three;
        while *position > 10 {
          *position -= 10;
        }

        scores[i] += *position;

        println!("Player {} casts {}+{}+{} and moves to {}, score {}",
          i, cast_one, cast_two, cast_three, *position, scores[i]);

        if scores[i] >= 1000 {
          break;
        }
      }
    }

    println!("\nFinal positions:");
    for (i, position) in positions.into_iter().enumerate() {
      println!("{}: {}", i, position);
    }
    println!("\nScores:");
    for (i, score) in scores.iter().enumerate() {
      println!("{}: {}", i, score);
    }
    println!("\nNumber of casts: {}", die.number_of_casts);
    println!("\nOutput: {}", die.number_of_casts * scores.into_iter().filter(|s| { *s < 1000 }).next().unwrap());
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