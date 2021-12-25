use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Package {
  Literal(u8, u8, u64),
  Operator(u8, u8, Vec<Package>),
}

fn read_single(bits: &Vec<u8>, idx: &mut usize) -> u8 {
  let value = bits[*idx];
  *idx += 1;
  value
}

fn read_triple(bits: &Vec<u8>, idx: &mut usize) -> u8 {
  let value: u8 = (bits[*idx] << 2) + (bits[*idx + 1] << 1) + bits[*idx + 2];
  *idx += 3;
  value
}

fn read_n(bits: &Vec<u8>, idx: &mut usize, len: usize) -> u32 {
  assert!(len <= 32);

  let mut result = 0_u32;
  for i in 0..len {
    result = (result << 1) + (bits[*idx + i] as u32)
  }

  *idx += len;

  result
}

fn parse_package(bits: &Vec<u8>, idx: &mut usize) -> Package {
  let version= read_triple(bits, idx);
  let type_identifier = read_triple(bits, idx);

  match type_identifier {
    4_u8 => {
      let mut value = 0_u64;

      loop {
        let should_continue = read_single(bits, idx) == 1_u8;

        value = (value << 4) + read_n(bits, idx, 4) as u64;

        if !should_continue {
          break;
        }
      }

      Package::Literal(version, type_identifier, value)
    },
    _ => {
      let length_type = read_single(bits, idx);
      let mut subpackages: Vec<Package> = Vec::new();

      // println!("{} - {} - {}", version, type_identifier, length_type);

      if length_type == 1_u8 {
        let length = read_n(bits, idx, 11);
        
        for _ in 0..length {
          subpackages.push(parse_package(bits, idx));
        }
      } else {
        let length = read_n(bits, idx, 15) as usize;
        let end_idx = *idx + length;

        while *idx < end_idx {
          subpackages.push(parse_package(bits, idx));
        }

        assert_eq!(*idx, end_idx);
      }

      Package::Operator(version, type_identifier, subpackages)
    }
  }
}

fn pkg_value(pkg: &Package /*, i: usize */) -> u64 {
  match pkg {
    Package::Literal(_, _, value) => (*value) as u64,
    Package::Operator(_, type_id, content) => {
      let values = content.iter().map(|p| pkg_value(p /*, i + 1 */)).collect::<Vec<_>>();
      let result: u64 = match type_id {
        0 => values.iter().sum::<u64>(),
        1 => values.iter().product::<u64>(),
        2 => *values.iter().min().unwrap(),
        3 => *values.iter().max().unwrap(),
        5 => if values[0] > values[1] { 1 } else { 0 },
        6 => if values[0] < values[1] { 1 } else { 0 },
        7 => if values[0] == values[1] { 1 } else { 0 },
        _ => unreachable!(),
      };

      // println!(
      //   "{}{} / {} -> {}",
      //   String::from(" ").repeat(i),
      //   type_id,
      //   values.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "),
      //   result,
      // );

      result
    },
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    for line in lines {
      let bits: Vec<u8> = line.unwrap().chars().map(|c| {
        let byte = c.to_digit(16).unwrap();

        (0..4).map(move |i| if (byte & (1 << (3 - i))) != 0 { 1_u8} else { 0_u8 })
      }).flatten().collect();

      let mut idx = 0_usize;
      let pkg = parse_package(&bits, &mut idx);

      let len = bits.len();
      for i in idx..len {
        if bits[i] != 0 {
          panic!("Expected trailing zeros, got a 1");
        }
      }

      println!("{}", pkg_value(&pkg/*, 0 */));
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