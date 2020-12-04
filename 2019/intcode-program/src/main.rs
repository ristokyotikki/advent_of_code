use std::convert::TryInto;

fn main() {
  let mut noun: usize = 0;
  let mut verb: usize = 0;
  let original_data: Vec<usize> = vec![
    1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 9, 1, 19, 1, 19, 5, 23, 1, 9, 23, 27, 2, 27,
    6, 31, 1, 5, 31, 35, 2, 9, 35, 39, 2, 6, 39, 43, 2, 43, 13, 47, 2, 13, 47, 51, 1, 10, 51, 55,
    1, 9, 55, 59, 1, 6, 59, 63, 2, 63, 9, 67, 1, 67, 6, 71, 1, 71, 13, 75, 1, 6, 75, 79, 1, 9, 79,
    83, 2, 9, 83, 87, 1, 87, 6, 91, 1, 91, 13, 95, 2, 6, 95, 99, 1, 10, 99, 103, 2, 103, 9, 107, 1,
    6, 107, 111, 1, 10, 111, 115, 2, 6, 115, 119, 1, 5, 119, 123, 1, 123, 13, 127, 1, 127, 5, 131,
    1, 6, 131, 135, 2, 135, 13, 139, 1, 139, 2, 143, 1, 143, 10, 0, 99, 2, 0, 14, 0,
  ];
  let mut data: Vec<usize> = original_data.clone();
  data[1] = noun;
  data[2] = verb;
  let mut current_sequence: i16 = 0;
  loop {
    let next = take_next(&current_sequence, &data);
    current_sequence += 4;

    match next[0] {
      1 => data[next[3]] = data[next[1]] + data[next[2]],
      2 => data[next[3]] = data[next[1]] * data[next[2]],
      99 => {
        if noun == 12 && verb == 2 {
          println!("answer 1 is: {}", data[0]);
        }

        if data[0] == 19690720 {
          println!("answer 2 is: {}", data[1] * 100 + data[2]);
          break;
        } else {
          if noun != 99 {
            noun += 1;
            current_sequence = 0;
          } else {
            noun = 0;
            verb += 1;
            current_sequence = 0;
            if verb == 99 {
              panic!("couldn't find target value of: 19690720");
            }
          }
          data = original_data.clone();
          data[1] = noun;
          data[2] = verb;
        }
      }
      _ => panic!("Error! Unexpected value: {:?}", next[0]),
    }
  }
}

fn take_next(curr: &i16, data: &Vec<usize>) -> Vec<usize> {
  let mut next: Vec<usize> = Vec::with_capacity(4);
  for number in 0..4 {
    let i: usize = (curr + number).try_into().unwrap();
    if i < data.len() {
      next.push(data[i]);
    }
  }
  next
}
