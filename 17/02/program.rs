use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct TargetArea {
  x_min: u32,
  x_max: u32,

  y_min: i32,
  y_max: i32,
}

fn parse_target_area(line: String) -> TargetArea {
  let prefix = "target area: ";
  if !line.starts_with(prefix) {
    panic!("expected target area");
  }

  let parts = line[prefix.len()..].split(", ");

  let mut x: Option<(u32, u32)> = None;
  let mut y: Option<(i32, i32)> = None;

  for part in parts {
    let subparts = part[2..].split("..").collect::<Vec<_>>();

    if part.starts_with("x=") {
      assert_eq!(subparts.len(), 2);

      x = Some((subparts[0].parse::<u32>().unwrap(), subparts[1].parse::<u32>().unwrap()));
    } else if part.starts_with("y=") {
      assert_eq!(subparts.len(), 2);
      
      y = Some((subparts[0].parse::<i32>().unwrap(), subparts[1].parse::<i32>().unwrap()));
    } else {
      panic!("invalid part {}", part);
    }
  }

  if let Some((x_min, x_max)) = x {
    if let Some((y_min, y_max)) = y {
      return TargetArea {
        x_min: x_min,
        x_max: x_max,
        y_min: y_min,
        y_max: y_max,
      };
    }
  }

  panic!("Expected x and y to be given");
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    for line in lines {
      let area = parse_target_area(line.unwrap());

      // the target area lies in the quadrant of x > 0 && y < 0
      assert!(area.x_min > 0);
      assert!(area.y_max < 0);
      
      // The ideal scenario always has a X velocity that ends up on zero inside
      // the target area, which means we don't actually need to consider X for
      // this excercise.

      // max velocity is where we end up at max value after 1 tick
      let max_x_velocity = area.x_max;
      // min velocity is the velocity which ends up on min_x after infinite ticks.
      // Computing n for which `min_x = n * (n+1) / 2` is hard to compute, pick a
      // lower bound that is definitey lower: min_x = (n + 1) ** 2 / 2
      let min_x_velocity = (area.x_min as f32 * 2_f32).sqrt().floor() as u32 - 1;

      // min velocity is the velocity which ends up at min value after 1 tick
      let min_y_velocity = area.y_min;
      // max velocity is the velocity that ends up going so high that when it
      // comes back down the point will land at min value the tick after hitting 0
      let max_y_velocity = -area.y_min - 1;

      let mut count = 0_usize;

      for start_x_velocity in min_x_velocity..(max_x_velocity + 1) {
        for start_y_velocity in min_y_velocity..(max_y_velocity + 1) {
          let (mut x, mut y) = (0_u32, 0_i32);
          let (mut x_velocity, mut y_velocity) = (start_x_velocity, start_y_velocity);

          while x <= area.x_max && y >= area.y_min {
            if x >= area.x_min && y <= area.y_max {
              println!("{}, {}", start_x_velocity, start_y_velocity);
              count += 1;
              break;
            }

            if x_velocity > 0 {
              x += x_velocity;
              x_velocity -= 1;
            }
            
            y += y_velocity;
            y_velocity -= 1;
          }
        }
      }

      println!("\n{}", count);
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