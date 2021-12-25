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

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let board: Vec<Vec<u32>> = lines.map(|line| {
      line.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect()
    }).collect();

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

      if new_paths.len() == 0 {
        break;
      }

      println!("{}", new_paths.len());
      paths = new_paths;
    }

    completed_paths.sort_unstable();

    println!("\n{}", completed_paths[0]);
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