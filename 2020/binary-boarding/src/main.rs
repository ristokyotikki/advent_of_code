use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
  let mut plane_rows = HashMap::<i32, Vec<i32>>::new();
  let mut highest_seat_num = 0;
  let file = File::open("inputs").expect("no such file");
  let buf = BufReader::new(file);
  let lines: Vec<String> = buf
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .collect();

  for line in lines.iter() {
    let mut row: i32 = 0;
    let mut low = 0;
    let mut high = 127;

    for (i, indicator) in line.chars().enumerate() {
      let new_vals = calc_seat(low, high, indicator);
      low = new_vals.0;
      high = new_vals.1;
      if high - low == 0 {
        match i {
          i if i == 6 => {
            row = low;
            low = 0;
            high = 7;
          }
          i if i == 9 => {
            let seat_num = row * 8 + low;
            if seat_num > highest_seat_num {
              highest_seat_num = seat_num;
            }

            match plane_rows.entry(row) {
              Entry::Vacant(e) => { e.insert(vec![seat_num]); },
              Entry::Occupied(mut e) => { e.get_mut().push(seat_num); }
            }
          }
          _ => {}
        }
      }
    }
  }
  println!("seat with the highest number is: {}", highest_seat_num);

  let mut dist = 0;
  let mut correct_row = 0;
  for row in plane_rows.iter() {
    if row.1.len() < 8 {
      if row.0 < &64 && row.0 > &dist {
        dist = *row.0;
        correct_row = *row.0;
      } else if row.0 >= &64 && (127 - row.0) > dist {
        dist = 127 - row.0;
        correct_row = *row.0;
      }
    }
  }

  let mut seat_id = 0;
  match plane_rows.get(&correct_row) {
    Some(seats) => {
      for i in 0..7 {
        if seats[i] != correct_row * 8 + i as i32 {
          seat_id = correct_row * 8 + i as i32;
        }
      }
    },
    None => println!("No row found! {}", correct_row),
  }

  println!("Your seat id is: {}", seat_id);
}

fn calc_seat(mut low: i32, mut high: i32, indicator: char) -> (i32, i32) {
  let difference = ((high as f64 - low as f64) / 2.0).ceil() as i32;

  match indicator {
    i if i == 'F' || i == 'L' => high = high - difference,
    i if i == 'B' || i == 'R' => low = low + difference,
    _ => println!("unexpected indicator! {}", indicator),
  }

  return (low, high);
}
