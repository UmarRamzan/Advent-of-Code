use std::fs;

fn main() {

    let file_path = String::from("input.txt");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut num_valid = 0;
    for line in lines {
        let elems: Vec<&str> = line.split(' ').collect();
        let min_max: Vec<&str> = elems[0].split('-').collect();
        let min = min_max[0].parse().unwrap();
        let max = min_max[1].parse().unwrap();

        let curr_char = elems[1].chars().nth(0).unwrap();

        let mut char_freq = 0;
        for c in elems[2].chars() {
            if c == curr_char {
                char_freq = char_freq + 1;
            }
        }

        if char_freq <= max && char_freq >= min {
            num_valid = num_valid + 1;

        }
    }

    println!("Solution: {num_valid}\n");

}