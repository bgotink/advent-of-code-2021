use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_line(line: &str) -> Vec<i32> {
  line.split(',').map(|x| x.parse::<i32>().unwrap()).collect()
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    for line in lines {
      let mut positions = parse_line(&line.unwrap());

      positions.sort_unstable();

      // The positions themselves don't actually matter, we just have to pick the median position
      let ideal = positions[positions.len() / 2];

      println!("ideal position: {}, count: {}", ideal, positions.len());
      
      let mut cost: i32 = 0;
      let mut precost: i32 = 0;
      let mut postcost: i32 = 0;
      for pos in positions {
        cost += (pos - ideal).abs();
        precost += (pos - ideal - 1).abs();
        postcost += (pos - ideal + 1).abs();
      }

      println!("cost: {} ({} - {})", cost, precost, postcost);
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