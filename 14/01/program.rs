use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

struct PairIter<'a, T> {
  arr: &'a Vec<T>,
  idx: usize,
}

impl <'a, T> PairIter<'a, T> {
  fn new(arr: &'a Vec<T>) -> PairIter<'a, T> {
    PairIter { arr: arr, idx: 1 }
  }
}

impl <'a, T> Iterator for PairIter<'a, T> where T: Copy {
  type Item = (T, T);

  fn next(&mut self) -> Option<(T, T)> {
    if self.idx >= self.arr.len() {
      None
    } else {
      let idx = self.idx;
      self.idx += 1;

      Some((self.arr[idx - 1], self.arr[idx]))
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut chain: Vec<char> = Vec::new();
    let mut rules: HashMap<(char, char), char> = HashMap::new();

    for line in lines {
      let l = line.unwrap();

      if l.len() == 0 {
        continue;
      }
      
      if chain.len() == 0 {
        chain = l.chars().collect();
      } else {
        let parts = l.split(" -> ").collect::<Vec<_>>();

        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].len(), 2);
        assert_eq!(parts[1].len(), 1);

        let from: Vec<char> = parts[0].chars().collect();
        let to = parts[1].chars().next().unwrap();

        rules.insert((from[0], from[1]), to);
      }
    }

    for i in 0..10 {
      let mut next_chain: Vec<char> = Vec::new();

      for (a, b) in PairIter::new(&chain) {
        if next_chain.len() == 0 {
          next_chain.push(a);
        }

        if let Some(extra) = rules.get(&(a, b)) {
          next_chain.push(*extra);
        }

        next_chain.push(b);
      }

      chain = next_chain;
      println!("Line after iteration {}: {}", i + 1, chain.iter().collect::<String>());
    }

    println!("");

    let mut counts: HashMap<char, usize> = HashMap::new();

    for c in chain {
      if counts.contains_key(&c) {
        *counts.get_mut(&c).unwrap() += 1;
      } else {
        counts.insert(c, 1);
      }
    }

    for (c, count) in counts.iter() {
      println!("{} => {}", c, count);
    }

    println!("");

    let mut min = usize::MAX;
    let mut max = 0_usize;

    for (_, count) in counts {
      if count < min {
        min = count;
      }
      if count > max {
        max = count;
      }
    }

    println!("min: {}, max: {}, diff: {}", min, max, max - min);
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