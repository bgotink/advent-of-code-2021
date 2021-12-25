use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
  let args: Vec<String> = env::args().collect();
  
  // Dirac die:
  // This die rolls 1, 2, and 3, splitting the universe into three parts.
  // Rolling three such dice means we end up in universes where the player
  // has rolled
  // - 3 (1, 1, 1) -> 1 time
  // - 4 (1, 1, 2) or (1, 2, 1) or (2, 1, 1) -> 3 times
  // - 5 (1, 1, 3) and variants or (2, 2, 1) and variants -> 6 times
  // - 6 (2, 2, 2) and all 6 permutations of (1, 2, 3) -> 7 times
  // - 7 (3, 3, 1) and variants or (3, 2, 2) and variants -> 6 times
  // - 8 (3, 3, 2) and variants -> 3 times
  // - 9 (3, 3, 3) -> 1 time
  // For a grand total of 27 universes per triple die roll
  let single_dirac_die_results: Vec<(u32, u64)> = vec![
    (3, 1),
    (4, 3),
    (5, 6),
    (6, 7),
    (7, 6),
    (8, 3),
    (9, 1)
  ];

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len());
  }

  if let Ok(lines) = read_lines(&args[1]) {
    let (start_player_1, start_player_2) = {
      let positions: Vec<u32> = lines.map(|line| { line.unwrap().chars().last().unwrap().to_digit(10).unwrap() }).collect();

      (*positions.iter().next().unwrap(), *positions.iter().last().unwrap())
    };

    // position_1, score_1, position_2, score_2, amount of universes
    let mut positions: Vec<(u32, u32, u32, u32, u64)> = Vec::new();
    positions.push((start_player_1, 0, start_player_2, 0, 1));
    
    let mut wins_player_one = 0_u64;
    let mut wins_player_two = 0_u64;

    while positions.len() > 0 {
      println!("Iterating over {} positions", positions.len());
      positions = positions.into_iter().flat_map(|(position_one, score_one, position_two, score_two, amount_of_universes)| {
        single_dirac_die_results.iter().map(move |(die_result_one, number_of_die_results)| {
          let mut new_position_one = position_one + *die_result_one;
          if new_position_one > 10 {
            new_position_one -= 10;
          }

          let new_score_one = score_one + new_position_one;
          let new_amount_of_universes = amount_of_universes * (*number_of_die_results);
          
          (new_position_one, new_score_one, position_two, score_two, new_amount_of_universes)
        })
      }).filter(|(_, score_one, _, _, amount_of_universes)| {
        if *score_one >= 21 {
          // println!("Player 1 has won {} universes", *amount_of_universes);
          wins_player_one += *amount_of_universes;
          false
        } else {
          true
        }
      }).flat_map(|(position_one, score_one, position_two, score_two, amount_of_universes)| {
        single_dirac_die_results.iter().map(move |(die_result_two, number_of_die_results)| {
          let mut new_position_two = position_two + *die_result_two;
          if new_position_two > 10 {
            new_position_two -= 10;
          }

          let new_score_two = score_two + new_position_two;          
          let new_amount_of_universes = amount_of_universes * (*number_of_die_results);

          (position_one, score_one, new_position_two, new_score_two, new_amount_of_universes)
        })
      }).filter(|(_, _, _, score_two, amount_of_universes)| {
        if *score_two >= 21 {
          // println!("Player 2 has won {} universes", *amount_of_universes);
          wins_player_two += *amount_of_universes;
          false
        } else {
          true
        }
      }).collect();
    }

    println!("\nNumber of wins:");
    println!("  Player 1 has won {} times", wins_player_one);
    println!("  Player 2 has won {} times", wins_player_two);
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