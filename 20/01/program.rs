use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_line(line: String) -> Vec<usize> {
  line.chars().map(|c| match c {
    '#' => 1_usize,
    '.' => 0_usize,
    _ => panic!("unexpected char {}", c),
  }).collect()
}

fn apply_algo(algo: &Vec<usize>, image: Vec<Vec<usize>>, iteration: usize) -> Vec<Vec<usize>> {
  let mut value_outside_bounds = 0_usize;

  for _ in 0..iteration {
    value_outside_bounds = if value_outside_bounds == 0 {
      algo[0]
    } else {
      algo[511]
    }
  }

  let max_y = image.len() as i32;
  let max_x = image.iter().next().unwrap().len() as i32;

  (-1..(max_y + 1)).map(|y| {
    (-1..(max_x + 1)).map(|x| {
      let mut idx = 0_usize;

      for y_mod in [-1, 0, 1] {
        for x_mod in [-1, 0, 1] {
          idx = (idx << 1) + if y + y_mod >= 0 && y + y_mod < max_y && x + x_mod >= 0 && x + x_mod < max_x {
            image[(y + y_mod) as usize][(x + x_mod) as usize]
          } else {
            value_outside_bounds
          };
        }
      }

      algo[idx]
    }).collect()
  }).collect()
}

fn print_image(img: &Vec<Vec<usize>>) {
  for line in img {
    println!(
      "{}",
      line.iter().map(|c| if *c > 0 { '#' } else { '.' }).collect::<String>(),
    );
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut algo: Vec<usize> = Vec::new();
    let mut image: Vec<Vec<usize>> = Vec::new();

    for l in lines {
      let line = l.unwrap();

      if line.len() == 0 {
        continue;
      } 
      
      if algo.len() == 0 {
        algo = parse_line(line);
      } else {
        image.push(parse_line(line));
      }
    }

    println!("{}x{} -> {}", image.len(), image.iter().next().unwrap().len(), image.iter().map(|line| line.iter().sum::<usize>()).sum::<usize>());
    // print_image(&image);

    image = apply_algo(&algo, image, 0);
    
    println!("{}x{} -> {}", image.len(), image.iter().next().unwrap().len(), image.iter().map(|line| line.iter().sum::<usize>()).sum::<usize>());
    // print_image(&image);
    
    image = apply_algo(&algo, image, 1);
    
    println!("{}x{} -> {}", image.len(), image.iter().next().unwrap().len(), image.iter().map(|line| line.iter().sum::<usize>()).sum::<usize>());
    // print_image(&image);
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