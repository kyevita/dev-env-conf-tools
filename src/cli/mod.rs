use std::{io::{Error, self}, path::PathBuf};

use home::home_dir;

#[derive(Debug, Copy, Clone)]
pub struct CLI {
  num_of_tries: u32
}

impl CLI {
  pub fn new() -> CLI {
    return CLI {
      num_of_tries: 0
    }
  }
  pub fn get_home_path(self) -> Result<PathBuf, &'static str> {
    match home_dir() {
      Some(path) => Ok(path),
      None => Err("Unable to get home directory")
    }
  }

  pub fn read_line(self) -> Result<String, Error> {
    let mut input = String::new();
    return match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(String::from(input.trim())),
        Err(error) => Err(error),
    }
  }

  pub fn read_line_until(mut self, expected: &str, tries: Option<u32>) -> bool {
    let input = self.read_line().unwrap();
    let mut max_tries = 5;
    
    if !tries.is_none() {
      max_tries = tries.unwrap();
    }

    if self.num_of_tries > max_tries {
      self.num_of_tries = 0;
      return false;
    }


    if input.eq(expected) {
      return true;
    }

    self.num_of_tries += 1;
    return self.read_line_until(expected, tries);
  }
}