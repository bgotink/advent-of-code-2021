use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BTreeMap;

fn parse_line(line: String) -> Vec<u32> {
  line.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let grid: Vec<_> = lines.map(|line| parse_line(line.unwrap())).collect();
    let mut basins: Vec<_> = grid.iter().map(|row| row.iter().map(|e| if *e == (9 as u32) { u32::MAX } else { 0 }).collect::<Vec<_>>()).collect();

    let max_y = grid.len();
    let max_x = grid.iter().next().unwrap().len();

    let mut current_basin = 1;
    for (y, row) in grid.iter().enumerate() {
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
        basins[y][x] = current_basin;
        current_basin += 1;
      }
    }

    loop {
      let mut changed = false;

      for y in 0..max_y {
        for x in 0..max_x {
          let val = basins[y][x];

          if val != 0 {
            continue;
          }

          if x > 0 && basins[y][x - 1] != 0 && basins[y][x - 1] != u32::MAX {
            basins[y][x] = basins[y][x - 1];
            changed = true;
          } else if x < max_x - 1 && basins[y][x + 1] != 0 && basins[y][x + 1] != u32::MAX {
            basins[y][x] = basins[y][x + 1];
            changed = true;
          } else if y > 0 && basins[y - 1][x] != 0 && basins[y - 1][x] != u32::MAX {
            basins[y][x] = basins[y - 1][x];
            changed = true;
          } else if y < max_y - 1 && basins[y + 1][x] != 0 && basins[y + 1][x] != u32::MAX {
            basins[y][x] = basins[y + 1][x];
            changed = true;
          }
        }
      }

      if !changed {
        break;
      }
    }

    let mut basin_sizes: BTreeMap<u32, u32> = BTreeMap::new();

    for row in basins.iter() {
      for basin in row.iter() {
        if basin_sizes.contains_key(&basin) {
          *basin_sizes.get_mut(basin).unwrap() += 1;
        } else {
          basin_sizes.insert(*basin, 1);
        }
      }
    }

    basin_sizes.remove(&0);
    basin_sizes.remove(&u32::MAX);

    println!("Basin sizes:");
    for (id, size) in basin_sizes.iter() {
      println!("{} -> {}", *id, *size);
    }

    let mut sizes: Vec<u32> = basin_sizes.values().map(|x| *x).collect();
    sizes.sort_unstable();

    println!("\n{}", sizes.iter().rev().take(3).product::<u32>());
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