use std::io::{self, Error};

#[derive(Debug, Copy, Clone)]
pub struct CLI {}

impl CLI {
    pub fn new() -> CLI {
        return CLI {};
    }

    pub fn read_line(self) -> Result<String, Error> {
        let mut input = String::new();
        return match io::stdin().read_line(&mut input) {
            Ok(_) => Ok(String::from(input.trim())),
            Err(error) => Err(error),
        };
    }

    pub fn read_line_until(
        mut self,
        expected: &str,
        max_tries: Option<i32>,
        start_at: Option<i32>,
    ) -> bool {
        let input = self.read_line().unwrap();
        let mut num_of_tries = match start_at {
            Some(n) => n,
            None => 0,
        };
        let mut tries: i32 = match max_tries {
            Some(t) => t,
            None => -1,
        };

        if num_of_tries > tries {
            return false;
        }

        if input.eq(expected) {
            return true;
        }

        num_of_tries += 1;
        return self.read_line_until(expected, Some(tries), Some(num_of_tries));
    }
}
