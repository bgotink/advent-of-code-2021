use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::hash::Hash;

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

fn count_char<K: Eq + Hash + Copy>(counts: &mut HashMap<K, usize>, c: &K, count: usize) -> () {
  if counts.contains_key(c) {
    *counts.get_mut(c).unwrap() += count;
  } else {
    counts.insert(*c, count);
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

    let mut counts: HashMap<(char, char), usize> = HashMap::new();

    for (a, b) in PairIter::new(&chain) {
      count_char(&mut counts, &(a, b), 1);
    }

    for _ in 0..40 {
      let mut new_counts: HashMap<(char, char), usize> = HashMap::new();

      for (pair, count) in counts {
        if let Some(extra) = rules.get(&pair) {
          let (a, b) = pair;

          count_char(&mut new_counts, &(a, *extra), count);
          count_char(&mut new_counts, &(*extra, b), count);
        } else {
          count_char(&mut new_counts, &pair, count);
        }
      }

      counts = new_counts;
    }

    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for ((a, b), count) in counts {
      count_char(&mut char_counts, &a, count);
      count_char(&mut char_counts, &b, count);
    }

    // We've counted every character twice, because we counted the start
    // and end of every pair, except for the first and last character in
    // the entire expanded chain, so we have to increase those by 1 before
    // halving the count.

    *char_counts.get_mut(chain.iter().last().unwrap()).unwrap() += 1;
    *char_counts.get_mut(chain.iter().next().unwrap()).unwrap() += 1;

    for (c, count) in char_counts.iter_mut() {
      *count = *count / 2;

      println!("{} => {}", c, count);
    }

    println!("");

    let mut min = usize::MAX;
    let mut max = 0_usize;

    for (_, count) in char_counts {
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