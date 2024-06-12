use std::fs;

fn main() {

    let file_path = String::from("input.txt");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut num_trees = 0;
    let mut x = 0;

    for line in lines.skip(1) {
        x = x + 3;
        let bytes = line.as_bytes();
        if bytes[x % bytes.len()] == b'#' {
            num_trees = num_trees + 1;
        }
    }

    println!("Solution: {num_trees}");
}
