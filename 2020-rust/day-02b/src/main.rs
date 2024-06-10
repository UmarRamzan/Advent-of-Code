use std::fs;

fn main() {

    let file_path = String::from("input.txt");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut num_valid = 0;
    for line in lines {

        let elems: Vec<&str> = line.split(' ').collect();
        let positions: Vec<&str> = elems[0].split('-').collect();

        let pos_one = positions[0].parse::<usize>().unwrap()-1;
        let pos_two = positions[1].parse::<usize>().unwrap()-1;

        let curr_char = elems[1].chars().nth(0).unwrap();

        let char_one = elems[2].chars().nth(pos_one).unwrap();
        let char_two = elems[2].chars().nth(pos_two).unwrap();

        let mut num_matches = 0;
        if char_one == curr_char {num_matches = num_matches + 1;}
        if char_two == curr_char {num_matches = num_matches + 1;}

        if num_matches == 1 {num_valid = num_valid + 1;}
    }

    println!("Solution: {num_valid}\n");

}