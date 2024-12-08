use std::fs::read_to_string;


pub fn get_input(day: u8)  -> Vec<String> {
    let mut filename = String::from("input/day_");
    filename.push_str(format!("{:02}.txt", day).as_str());
    return read_lines(&filename);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
