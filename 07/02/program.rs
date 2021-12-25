use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{BTreeSet,BTreeMap};

fn parse_line(line: &str) -> Vec<i32> {
  line.split(',').map(|x| x.parse::<i32>().unwrap()).collect()
}

fn find_ideal(positions: &Vec<i32>) -> Option<(i32, i32)> {
  let unique_positions: BTreeSet<i32> = positions.into_iter().map(|x| *x).collect();
  let mut position_counts: BTreeMap<i32, i32> = (&unique_positions).into_iter().map(|x| (*x, 0)).collect();

  for pos in positions {
    *position_counts.get_mut(&pos).unwrap() += 1;
  }

  let min = *unique_positions.iter().next().unwrap();
  let max = *unique_positions.iter().last().unwrap();

  let increasing: Vec<i32> = (min..max).map(|x| {
    position_counts.iter().filter(|(y, _)| **y < x).map(|(y, c)| {
      let distance = x - *y;
      
      c * distance * (distance + 1) / 2
    }).sum()
  }).collect();

  let decreasing: Vec<i32> = (min..max).map(|x| {
    position_counts.iter().filter(|(y, _)| **y > x).map(|(y, c)| {
      let distance = *y - x;
      
      c * distance * (distance + 1) / 2
    }).sum()
  }).collect();

  let mut previous_pos = 0;
  let mut previous_cost = i32::MAX;
  for (i, pos) in (min..max).enumerate() {
    let cost = increasing[i] + decreasing[i];

    // if increasing[i] > decreasing[i] {
    //   // We've found the middle, but on which side of the middle does the ideal reside?

      if cost > previous_cost {
        return Some((previous_pos, previous_cost))
      }

    //   return Some((pos, cost))
    // }

    previous_cost = cost;
    previous_pos = pos;
  }

  None
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
      if let Some((ideal, cost)) = find_ideal(&positions) {
        println!("ideal position: {}, cost: {}", ideal, cost);
      } else {
        panic!("Failed to find ideal position");
      }
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