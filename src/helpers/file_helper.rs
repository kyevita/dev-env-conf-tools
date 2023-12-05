use std::{fs::File, io::BufReader, io::BufRead};

pub fn search_line_by_string(file: &File, string_to_search: String) -> Option<String> {
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_str = line.unwrap();

        if line_str.contains(&string_to_search) {
            return Some(line_str);
        }
    }

    return None;
}