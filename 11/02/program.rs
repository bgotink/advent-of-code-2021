use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut board: Vec<Vec<u32>> = lines.map(|line| {
      line.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect()
    }).collect();

    let max_y = board.len();
    let max_x = board.iter().next().unwrap().len();

    let nb_cells = max_y * max_x;
    
    let mut i = 0;
    loop {
      i += 1;
      
      let mut flashes: usize = 0;
      let mut flashed: Vec<Vec<bool>> = board.iter().map(|row| row.iter().map(|_| false).collect()).collect();

      for row in board.iter_mut() {
        for val in row.iter_mut() {
          *val += 1;
        }
      }
      
      let mut changed = true;
      while changed {
        changed = false;

        for y in 0..max_y {
          for x in 0..max_x {
            if board[y][x] > 9 && !flashed[y][x] {
              changed = true;
              flashed[y][x] = true;
              flashes += 1;

              if x > 0 {
                board[y][x - 1] += 1;
              }
              if x < max_x - 1 {
                board[y][x + 1] += 1;
              }

              if y > 0 {
                board[y - 1][x] += 1;

                if x > 0 {
                  board[y - 1][x - 1] += 1;
                }
                if x < max_x - 1 {
                  board[y - 1][x + 1] += 1;
                }
              }
              if y < max_y - 1 {
                board[y + 1][x] += 1;

                if x > 0 {
                  board[y + 1][x - 1] += 1;
                }
                if x < max_x - 1 {
                  board[y + 1][x + 1] += 1;
                }
              }
            }
          }
        }
      }

      for y in 0..max_y {
        for x in 0..max_x {
          if flashed[y][x] {
            board[y][x] = 0;
          }
        }
      }

      if flashes == nb_cells {
        println!("{}", i);
        return;
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