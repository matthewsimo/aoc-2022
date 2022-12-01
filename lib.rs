use std::fs;

pub fn process_input(input: &str) -> String {
    // let file = fs::read_to_string(input).unwrap();
    let result = input.split("\n");
    return result.unwrap().to_string();
}
