use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{BTreeMap,BTreeSet,HashMap};

#[derive(Copy,Clone,Eq,PartialEq)]
enum SmallCaveCount {
  Free,
  Ready(usize),
  Used,
}

fn nb_paths(
  large_caves: &BTreeSet<usize>,
  connections: &BTreeMap<usize, Vec<usize>>,
  previous_positions: &mut BTreeSet<usize>,
  double: SmallCaveCount,
  position: usize,
  end: usize,
) -> usize {
  let mut count = 0;

  if let Some(conn) = connections.get(&position) {
    for next in conn {
      match *next {
        0 => (),
        _ => {
          if *next == end {
            if double == SmallCaveCount::Free || double == SmallCaveCount::Used {
              count += 1;
            } // else: path doesn't have a double, so it's identical to a path we've already counted with SmallCaveCount::Free
            continue;
          }

          let is_large = large_caves.contains(next);
          let is_double_visit = previous_positions.contains(next);
          let can_visit = is_large || !is_double_visit || match double {
            SmallCaveCount::Ready(double_pos) => double_pos == *next,
            _ => false
          };

          if can_visit {
            if !is_large && !is_double_visit {
              previous_positions.insert(*next);
            }
  
            count += nb_paths(
              large_caves,
              connections,
              previous_positions,
              if is_double_visit { SmallCaveCount::Used } else { double },
              *next,
              end,
            );

            if !is_double_visit && double == SmallCaveCount::Free {
              count += nb_paths(
                large_caves,
                connections,
                previous_positions,
                SmallCaveCount::Ready(*next),
                *next,
                end,
              );
            }
    
            if !is_large && !is_double_visit {
              previous_positions.remove(next);
            }
          }
        }
      }
    }
  }

  count
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let mut end_pos: Option<usize> = None;
    
    let mut positions: HashMap<String, usize> = HashMap::new();
    positions.insert("start".into(), 0);

    let mut large_caves: BTreeSet<usize> = BTreeSet::new();
    let mut connections: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    for line in lines {
      let l = line.unwrap();
      let parts = l.split('-').collect::<Vec<&str>>();

      assert_eq!(parts.len(), 2);

      let start_str: String = parts[0].into();
      let end_str: String = parts[1].into();

      let start: usize;
      if positions.contains_key(&start_str) {
        start = *positions.get(&start_str).unwrap();
      } else {
        start = positions.len();
        
        if start_str.to_ascii_uppercase() == start_str {
          large_caves.insert(start);
        }

        positions.insert(start_str, start);
      }

      let end: usize;
      if positions.contains_key(&end_str) {
        end = *positions.get(&end_str).unwrap();
      } else {
        end = positions.len();

        if end_str.to_ascii_uppercase() == end_str {
          large_caves.insert(end);
        }

        if end_pos.is_none() && end_str == "end" {
          end_pos = Some(end);
        }

        positions.insert(end_str, end);
      }

      if let Some(conn) = connections.get_mut(&start) {
        conn.push(end);
      } else {
        connections.insert(start, vec![end]);
      }

      if let Some(conn) = connections.get_mut(&end) {
        conn.push(start);
      } else {
        connections.insert(end, vec![start]);
      }
    }

    println!("Indices:");
    for (name, idx) in positions.iter() {
      println!("  {} -> {}", name, idx);
    }

    println!("\nEnd: {}", end_pos.unwrap());

    println!("\nConnections:");
    for (from, to) in connections.iter() {
      println!("  {} -> {}", from, to.iter().map(|v| v.to_string() + ", ").collect::<String>());
    }

    println!("\nLarge caves:");
    for cave in large_caves.iter() {
      println!("- {}", cave);
    }
    println!("");
    
    println!(
      "{}", 
      nb_paths(&large_caves, &connections, &mut BTreeSet::from([0]), SmallCaveCount::Free, 0, end_pos.unwrap()),
    );
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