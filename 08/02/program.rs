use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Display {
  one: u32,
  four: u32,
}

impl Display {
  fn map_to_number(val: &String) -> u32 {
    let channels = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let mut result: u32 = 0;

    for c in val.chars() {
      result += 1 << channels.iter().position(|x| *x == c).unwrap();
    }

    result
  }

  fn new(one: &String, four: &String) -> Display {
    Display {
      one: Display::map_to_number(one),
      four: Display::map_to_number(four),
    }
  }

  fn to_number(&self, val: &String) -> Option<u32> {
    let nb_val = Display::map_to_number(val);

    match nb_val.count_ones() {
      2 => return Some(1),
      3 => return Some(7),
      4 => return Some(4),
      7 => return Some(8),
      _ => ()
    }

    match (nb_val & self.one).count_ones() {
      1 => {
        // Either 2, 5, or 6

        if nb_val.count_ones() == 6 {
          // 6 is the only one of the three digits that has 6 segments
          return Some(6);
        }

        match (nb_val & self.four).count_ones() {
          2 => Some(2),
          3 => Some(5),
          _ => None,
        }
      },
      2 => {
        // Either 3, 9 or 0

        if nb_val.count_ones() == 5 {
          // 3 is the only of the three digits that has only 5 segments
          return Some(3);
        }

        match (nb_val & self.four).count_ones() {
          3 => Some(0),
          4 => Some(9),
          _ => None,
        }
      },
      _ => None,
    }
  }
}

fn parse_line(line: &str) -> (Vec<String>, Vec<String>) {
  let parts: Vec<_> = line.split(" | ").collect();

  if parts.len() != 2 {
    panic!("invalid line");
  }

  (
    parts[0].split_whitespace().map(|x| String::from(x)).collect(),
    parts[1].split_whitespace().map(|x| String::from(x)).collect()
  )
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut total = 0;

    for line in lines {
      let (dummy, output) = parse_line(&line.unwrap());

      let mut one: Option<&String> = None;
      let mut four: Option<&String> = None;

      for s in dummy.iter().chain(output.iter()) {
        match s.len() {
          2 => one = Some(s),
          4 => four = Some(s),
          _ => (),
        }
      }

      let display = Display::new(one.unwrap(), four.unwrap());

      let mut output_val = 0;
      for s in output {
        output_val = 10 * output_val + display.to_number(&s).unwrap();
      }

      println!("{}", output_val);

      total += output_val;
    }
    
    println!("\n{}", total);
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