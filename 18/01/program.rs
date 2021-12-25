use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::convert::TryInto;

enum SnailfishNumber {
  Single(u8),
  Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

impl std::fmt::Display for SnailfishNumber {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SnailfishNumber::Single(v) => write!(f, "{}", v),
      SnailfishNumber::Pair(left, right) => write!(f, "[{},{}]", (*left), (*right)),
    }
  }
}

impl SnailfishNumber {
  fn parse(str: String) -> Option<SnailfishNumber> {
    SnailfishNumber::_parse(&mut str.chars())
  }

  fn _parse<'a>(str: &mut std::str::Chars<'a>) -> Option<SnailfishNumber> {
    let v = str.next()?;

    match v {
      '[' => {
        let left = SnailfishNumber::_parse(str)?;
        if str.next()? != ',' {
          return None
        }
        let right = SnailfishNumber::_parse(str)?;
        if str.next()? != ']' {
          return None
        }

        Some(SnailfishNumber::Pair(Box::new(left), Box::new(right)))
      },
      _ => Some(SnailfishNumber::Single(v.to_digit(10)?.try_into().ok()?))
    }
  }

  fn find_leftmost_number(&mut self) -> &mut u8 {
    match self {
      SnailfishNumber::Single(ref mut v) => v,
      SnailfishNumber::Pair(ref mut left, _) => (*left).find_leftmost_number(),
    }
  }

  fn find_rightmost_number(&mut self) -> &mut u8 {
    match self {
      SnailfishNumber::Single(ref mut v) => v,
      SnailfishNumber::Pair(_, ref mut right) => (*right).find_rightmost_number(),
    }
  }

  fn single_val(&self) -> Option<u8> {
    match self {
      SnailfishNumber::Single(v) => Some(*v),
      _ => None
    }
  }

  fn explode(&mut self) -> Option<bool> {
    let mut ignored_left = 0_u8;
    let mut ignored_right = 0_u8;

    self._explode(0, &mut ignored_left, &mut ignored_right)
  }

  fn _explode(&mut self, depth: u8, left: &mut u8, right: &mut u8) -> Option<bool> {
    if let SnailfishNumber::Pair(ref mut a, ref mut b) = self {
      if depth < 3 {
        if (*a)._explode(depth + 1, left, (*b).find_leftmost_number())? {
          return Some(true);
        }

        return Some((*b)._explode(depth + 1, (*a).find_rightmost_number(), right)?);
      } else {
        // We're at depth 4, if a or b are pairs they need to explode
        if let SnailfishNumber::Pair(a1, a2) = &**a {
          *left += (*a1).single_val()?;
          *(*b).find_leftmost_number() += (*a2).single_val()?;
          
          **a = SnailfishNumber::Single(0);
          Some(true)
        } else if let SnailfishNumber::Pair(b1, b2) = &**b {
          *(*a).find_rightmost_number() += (*b1).single_val()?;
          *right += (*b2).single_val()?;
          
          **b = SnailfishNumber::Single(0);
          Some(true)
        } else {
          Some(false)
        }
      }
    } else {
      Some(false)
    }
  }

  fn split(&mut self) -> Option<bool> {
    if let SnailfishNumber::Pair(ref mut a, ref mut b) = self {
      match **a {
        SnailfishNumber::Single(v) => {
          if v >= 10 {
            **a = (*a)._split()?;
            return Some(true)
          }
        },
        SnailfishNumber::Pair(_, _) => {
          if (*a).split()? {
            return Some(true)
          }
        }
      }

      match **b {
        SnailfishNumber::Single(v) => {
          if v >= 10 {
            **b = (*b)._split()?;
            Some(true)
          } else {
            Some(false)
          }
        },
        SnailfishNumber::Pair(_, _) => {
          (*b).split()
        }
      }
    } else {
      Some(false)
    }
  }

  fn _split(&self) -> Option<SnailfishNumber> {
    match self {
      SnailfishNumber::Single(v) => {
        let v_f = *v as f32;
        let left = (v_f / 2_f32).floor() as u8;
        let right = (v_f / 2_f32).ceil() as u8;

        Some(
          SnailfishNumber::Pair(
            Box::new(SnailfishNumber::Single(left)),
            Box::new(SnailfishNumber::Single(right)),
          )
        )
      },
      _ => None,
    }
  }

  fn reduce(&mut self) -> Option<()> {
    println!("reducing {}", self);
    loop {
      if self.explode()? {
        println!("exploded to {}", self);
        continue;
      }

      if self.split()? {
        println!("split to {}", self);
        continue;
      }

      break;
    }

    Some(())
  }

  fn magnitude(&self) -> u64 {
    match self {
      SnailfishNumber::Single(v) => *v as u64,
      SnailfishNumber::Pair(a, b) => 3 * (*a).magnitude() + 2 * (*b).magnitude(),
    }
  }
}

impl std::ops::Add for SnailfishNumber {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    SnailfishNumber::Pair(
      Box::new(self),
      Box::new(other),
    )
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut num: Option<SnailfishNumber> = None;

    for line in lines {
      println!("");
      let line_num = SnailfishNumber::parse(line.unwrap()).unwrap();

      num = match num {
        Some(n) => {
          let mut new_num = n + line_num;

          new_num.reduce().unwrap();

          Some(new_num)
        },
        None => Some(line_num),
      }
    }

    if let Some(n) = num {
      println!("\n{}", &n);
      println!("\n{}", n.magnitude());
    } else {
      panic!("expected a result");
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