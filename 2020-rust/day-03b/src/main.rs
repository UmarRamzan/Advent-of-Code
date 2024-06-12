use std::fs;

fn main() {

    let file_path = String::from("input.txt");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let steps: [usize; 5] = [1, 3, 5, 7, 1];
    let mut curr_pos: [usize; 5] = [0, 0, 0, 0, 0];
    let mut num_trees: [usize; 5] = [0, 0, 0, 0, 0];

    let mut pos_y = 1;

    for line in lines.skip(1) {
        let bytes = line.as_bytes();
        for (i, step) in steps.iter().enumerate() {
            if i == 4 && (pos_y % 2 == 0 && pos_y != 0) {
                curr_pos[i] = curr_pos[i] + step;
                if bytes[curr_pos[i] % bytes.len()] == b'#' {
                    num_trees[i] = num_trees[i] + 1;
                }
            } else if i != 4 {
                curr_pos[i] = curr_pos[i] + step;
                if bytes[curr_pos[i] % bytes.len()] == b'#' {
                    num_trees[i] = num_trees[i] + 1;
                }
            }
        }
        pos_y = pos_y + 1;
    }

    let mut sol = 1;
    for n in num_trees { sol = sol * n; }

    println!("Solution: {sol}");
}
