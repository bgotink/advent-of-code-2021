use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_line(line: String) -> Vec<u32> {
  line.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut total_risk = 0;

    let grid: Vec<_> = lines.map(|line| parse_line(line.unwrap())).collect();

    let max_y = grid.len();
    for (y, row) in grid.iter().enumerate() {
      let max_x = row.len();

      for (x, val) in row.iter().enumerate() {
        if x > 0 && row[x - 1] <= *val {
          continue;
        }

        if x < max_x - 1 && row[x + 1] <= *val {
          continue;
        }

        if y > 0 && grid[y - 1][x] <= *val {
          continue;
        }

        if y < max_y - 1 && grid[y + 1][x] <= *val {
          continue;
        }

        println!("Found minimum at ({}, {}) with value {}", x, y, *val);
        total_risk += *val + 1;
      }
    }
    
    println!("\n{}", total_risk);
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