use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn find_bounds(board: &Vec<Vec<u32>>) -> u32 {
  let (mut one, mut two) = (0_u32, 0_u32);

  let size = board.len();
  for i in 1..size {
    one += board[i][0] + board[size - 1][i];
    two += board[0][i] + board[i][size - 1];
  }

  if one < two {
    one
  } else {
    two
  }
}

fn increase(base: u32, m: u32) -> u32 {
  if base + m > 9 {
    base + m - 9
  } else {
    base + m
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let tile: Vec<Vec<u32>> = lines.map(|line| {
      line.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect()
    }).collect();

    let board: Vec<Vec<u32>>;
    
    {
      let board_line: Vec<_> = tile.iter().map(|line| {
        let mut tile_line: Vec<u32> = Vec::new();

        for m in 0..5 {
          for cell in line.iter() {
            tile_line.push(increase(*cell, m));
          }
        }

        tile_line
      }).collect();

      let mut _board: Vec<Vec<u32>> = Vec::new();
      for m in 0..5 {
        for line in board_line.iter() {
          _board.push(
            line.iter().map(|cell| increase(*cell, m)).collect()
          );
        }
      }

      board = _board;
    }

    let mut bounds: Vec<Vec<u32>> = board.iter().map(|line| {
      line.iter().map(|_| 0).collect()
    }).collect();

    let size = board.len();
    let bound = find_bounds(&board);

    let mut paths: Vec<(usize, usize, u32)> = Vec::new();
    let mut completed_paths: Vec<u32> = Vec::new();
    paths.push((0, 0, 0));

    loop {
      let mut new_paths: Vec<(usize, usize, u32)> = Vec::new();

      for (x, y, risk) in paths {
        if (risk as usize) + (size - 1 - x) + (size - 1 - y) > (bound as usize) {
          continue;
        }

        if x == size - 1 && y == size - 1 {
          completed_paths.push(risk);
          continue;
        }

        // Look in the four directions and add those to the list for the next iteration.
        // Only do so if the new cumulative risk is the lowest (or first) we've seen for
        // that location, as there's no point in continuing down a path if we have already
        // reached that location with lower risk.

        if x > 0 {
          let new_risk = risk + board[y][x - 1];
          if new_risk <= bound && (bounds[y][x - 1] == 0 || bounds[y][x - 1] > new_risk) {
            bounds[y][x - 1] = new_risk;
            new_paths.push((x - 1, y, new_risk));
          }
        }
        if x < size - 1 {
          let new_risk = risk + board[y][x + 1];
          if new_risk <= bound && (bounds[y][x + 1] == 0 || bounds[y][x + 1] > new_risk) {
            bounds[y][x + 1] = new_risk;
            new_paths.push((x + 1, y, new_risk));
          }
        }

        if y > 0 {
          let new_risk = risk + board[y - 1][x];
          if new_risk <= bound && (bounds[y - 1][x] == 0 || bounds[y - 1][x] > new_risk) {
            bounds[y - 1][x] = new_risk;
            new_paths.push((x, y - 1, new_risk));
          }
        }
        if y < size - 1 {
          let new_risk = risk + board[y + 1][x];
          if new_risk <= bound && (bounds[y + 1][x] == 0 || bounds[y + 1][x] > new_risk) {
            bounds[y + 1][x] = new_risk;
            new_paths.push((x, y + 1, new_risk));
          }
        }
      }

      // We already only include paths if they are the lowest risk, but that doesn't stop
      // us from registering two paths if the second is shorter than the first, because
      // both will be the shortest at the time of their registration.
      // This extra filter prevents that, and ensures that at most one path per cell in
      // the cave matrix is present per iteration.
      // This effectively limits us to n * n calculations per iteration, dramatically
      // decreasing computation time.
      new_paths = new_paths.into_iter().filter(|(x, y, risk)| {
        bounds[*y][*x] == *risk
      }).collect();

      if new_paths.len() == 0 {
        break;
      }

      paths = new_paths;
    }

    completed_paths.sort_unstable();

    println!("{}", completed_paths[0]);
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