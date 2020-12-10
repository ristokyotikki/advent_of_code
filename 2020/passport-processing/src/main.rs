use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Passport {
  byr: bool,
  cid: bool,
  ecl: bool,
  eyr: bool,
  hcl: bool,
  hgt: bool,
  iyr: bool,
  pid: bool,
}

impl Passport {
  fn valid(&self) -> bool {
    return self.byr && self.ecl && self.eyr && self.hcl && self.hgt && self.iyr && self.pid;
  }

  fn clear(&mut self) {
    self.byr = false;
    self.cid = false;
    self.ecl = false;
    self.eyr = false;
    self.hcl = false;
    self.hgt = false;
    self.iyr = false;
    self.pid = false;
  }

  fn validate_byr(&mut self, byr: String) {
    let birth_year: usize = byr.parse().unwrap();
    self.byr = byr.len() == 4 && birth_year >= 1920 && birth_year <= 2002;
  }

  fn validate_iyr(&mut self, iyr: String) {
    let issue: usize = iyr.parse().unwrap();
    self.iyr = iyr.len() == 4 && issue >= 2010 && issue <= 2020;
  }

  fn validate_eyr(&mut self, eyr: String) {
    let exp: usize = eyr.parse().unwrap();
    self.eyr = eyr.len() == 4 && exp >= 2020 && exp <= 2030;
  }

  fn validate_hgt(&mut self, hgt: String) {
    let measure: String = hgt.chars().rev().collect();
    let height: String = measure[2..].chars().rev().collect();
    let measure: String = measure[..2].chars().rev().collect();
    if measure == "cm" {
      let height: usize = height.parse().unwrap();
      self.hgt = height >= 150 && height <= 193;
    } else if measure == "in" {
      let height: usize = height.parse().unwrap();
      self.hgt = height >= 59 && height <= 76;
    }
  }

  fn validate_hcl(&mut self, hcl: String) {
    if hcl.chars().nth(0).unwrap() == '#' && hcl[1..].len() == 6 {
      let re = Regex::new(r"^[a-f0-9]+").unwrap();
      self.hcl = re.is_match(&hcl[1..]);
    }
  }

  fn validate_ecl(&mut self, ecl: String) {
    self.ecl = [
      "amb".to_string(),
      "blu".to_string(),
      "brn".to_string(),
      "gry".to_string(),
      "grn".to_string(),
      "hzl".to_string(),
      "oth".to_string(),
    ]
    .contains(&ecl);
  }

  fn validate_pid(&mut self, pid: String) {
    if pid.len() == 9 {
      let re = Regex::new(r"^[0-9]+").unwrap();
      self.pid = re.is_match(&pid);
    }
  }
}

fn main() {
  println!("\n------------------------------------");
  println!("Task 1: In your batch file, how many passports are valid?\n");
  task_one();
  println!("\n------------------------------------");
  println!("Task 2: In your batch file, how many passports are valid?\n");
  task_two();
  println!("");
}

fn task_one() {
  let mut valid_passports = 0;
  let mut passport = Passport {
    byr: false,
    cid: false,
    ecl: false,
    eyr: false,
    hcl: false,
    hgt: false,
    iyr: false,
    pid: false,
  };
  let file = File::open("inputs").expect("no such file");
  let buf = BufReader::new(file);
  let lines: Vec<String> = buf
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .collect();

  for (i, line) in lines.iter().enumerate() {
    let fields = line.split_whitespace();

    for field in fields {
      match &field[..3] {
        "byr" => passport.byr = true,
        "ecl" => passport.ecl = true,
        "eyr" => passport.eyr = true,
        "hcl" => passport.hcl = true,
        "hgt" => passport.hgt = true,
        "iyr" => passport.iyr = true,
        "pid" => passport.pid = true,
        "cid" => passport.cid = true,
        _ => println!("unexpected str! {}", &field[..3]),
      }
    }

    if line.len() == 0 || i == lines.len() - 1 {
      if passport.valid() {
        valid_passports += 1;
      }
      passport.clear();
      continue;
    }
  }

  println!("Number of valid passports is: {}.", valid_passports);
}

fn task_two() {
  let mut valid_passports = 0;
  let mut passport = Passport {
    byr: false,
    cid: false,
    ecl: false,
    eyr: false,
    hcl: false,
    hgt: false,
    iyr: false,
    pid: false,
  };
  let file = File::open("inputs").expect("no such file");
  let buf = BufReader::new(file);
  let lines: Vec<String> = buf
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .collect();

  for (i, line) in lines.iter().enumerate() {
    let fields = line.split_whitespace();

    for field in fields {
      match &field[..3] {
        "byr" => passport.validate_byr(field[4..].to_string()),
        "ecl" => passport.validate_ecl(field[4..].to_string()),
        "eyr" => passport.validate_eyr(field[4..].to_string()),
        "hcl" => passport.validate_hcl(field[4..].to_string()),
        "hgt" => passport.validate_hgt(field[4..].to_string()),
        "iyr" => passport.validate_iyr(field[4..].to_string()),
        "pid" => passport.validate_pid(field[4..].to_string()),
        "cid" => passport.cid = true,
        _ => println!("unexpected str! {}", &field[..3]),
      }
    }

    if line.len() == 0 || i == lines.len() - 1 {
      if passport.valid() {
        valid_passports += 1;
      }
      passport.clear();
      continue;
    }
  }

  println!("Number of valid passports is: {}.", valid_passports);
}
