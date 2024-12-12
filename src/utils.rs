use std::fs::read_to_string;


pub fn get_input(day: u8)  -> Vec<String> {
    let filename = format!("input/day_{:02}.txt", day);
    read_lines(&filename)
}

fn read_lines(filename: &str) -> Vec<String> {
    let input_str = read_to_string(filename)
        .expect(&format!("Error reading file {}", filename));
    
    input_str.lines()
    .map(|line| line.to_string())
    .collect()
}
