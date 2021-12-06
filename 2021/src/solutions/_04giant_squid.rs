use crate::solutions::utils::*;

#[derive(Debug, Clone)]
struct BingoSheet {
  board: [usize; 25],
  hits: Vec<usize>,
  won: bool
}

impl BingoSheet {
  fn mark(mut self, input: usize) -> Self {
    if self.board.contains(&input) && self.won == false {
      let index = self.board.iter().position(|&n| n == input).unwrap();
      self.hits.push(index);
    }

    return BingoSheet {
      board: self.board,
      hits: self.hits,
      won: self.won
    };
  }

  fn num_of_hits(&self) -> usize {
    return self.hits.len();
  }

  fn check_bingo(&mut self) -> bool {
    if self.won {
      return false;
    }

    let pos = [0, 5, 10, 15, 20, 0, 1, 2, 3, 4];
    for (index, start_pos) in pos.iter().enumerate() {
      if self.hits.contains(start_pos) {
        let mut hits = 1;
        let mut no_match = false;
        let mut incr = 1;
        if index > 4 {
          incr = 5;
        }
        let mut next = start_pos + incr;
        while no_match == false {
          if self.hits.contains(&next) {
            hits += 1;
            if hits == 5 {
              self.won = true;
              return true;
            }

            next += incr;
          } else {
            no_match = true;
          }
        }
      }
    }

    return false;
  }
}

pub fn run() {
  println!("\nExcuting Day 4 Giant Squid - excercise 1");
  excercise_one();

  println!("\nExcuting Day 4 Giant Squid - excercise 2");
  excercise_two();
}

fn excercise_two() {
  let bingo_sheets_and_draws = create_bingo_sheets_and_draws();
  let draws = bingo_sheets_and_draws.0;
  let mut bingo_sheets = bingo_sheets_and_draws.1.clone();
  let mut winning_draw = 0;
  let mut winning_board = BingoSheet {
    board: [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    hits: Vec::new(),
    won: false
  };

  for draw in draws {
    bingo_sheets = bingo_sheets
      .iter()
      .map(|b| {
        let mut cloned_board = b.clone().mark(draw);
        if cloned_board.num_of_hits() >= 5 {
          if cloned_board.check_bingo() {
            winning_board = cloned_board.clone();
            winning_draw = draw;
          }
        }

        return cloned_board;
      })
      .collect();
  }
  calculate_bingo_answer(winning_board, winning_draw);
}

fn excercise_one() {
  let bingo_sheets_and_draws = create_bingo_sheets_and_draws();
  let draws = bingo_sheets_and_draws.0;
  let mut bingo_sheets = bingo_sheets_and_draws.1.clone();
  let mut found_winner = false;

  for draw in draws {
    bingo_sheets = bingo_sheets
      .iter()
      .map(|b| {
        let mut cloned_board = b.clone().mark(draw);
        if found_winner {
          return cloned_board;
        }

        if cloned_board.num_of_hits() >= 5 {
          if cloned_board.check_bingo() {
            calculate_bingo_answer(cloned_board.clone(), draw);
            found_winner = true;
          }
        }

        return cloned_board;
      })
      .collect();
  }
}

fn calculate_bingo_answer(winning_sheet: BingoSheet, draw: usize) {
  let mut total = 0;

  for (i, n) in winning_sheet.board.iter().enumerate() {
    if !winning_sheet.hits.contains(&i) {
      total += n;
    }
  }

  println!("Winning bingo result is {}", total * draw);
}

fn create_bingo_sheets_and_draws() -> (Vec<usize>, Vec<BingoSheet>) {
  let lines = read_lines_from_file("src/inputs/giant_squid");
  let mut i = 0;
  let mut draws: Vec<usize> = Vec::new();
  let mut bingo_sheets: Vec<BingoSheet> = Vec::new();
  let mut new_board: [usize; 25] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  ];

  for (index, line) in lines.iter().enumerate() {
    match index {
      0 => {
        draws = line
          .split(",")
          .map(|n| n.parse::<usize>().unwrap())
          .collect()
      }
      n if n == 1 || (n - 1) % 6 == 0 => {
        let new_bingo_board = BingoSheet {
          board: new_board,
          hits: Vec::new(),
          won: false
        };
        bingo_sheets.push(new_bingo_board);
        i = 0;
      }
      _ => {
        line.split_whitespace().for_each(|n| {
          new_board[i] = n.parse::<usize>().unwrap();
          i += 1;
        });
      }
    }
  }

  return (draws, bingo_sheets);
}
