use std::fs;

fn main() {

    let file_path = String::from("input.txt");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut nums: Vec<i32> = Vec::new();

    let lines = contents.split("\n");
    for line in lines {
        let converted_int: i32 = line.parse().unwrap();
        nums.push(converted_int)
    }

    let sorted_vec = merge_sort(&nums);

    let (num_one, num_two, num_three) = find_nums(&sorted_vec, 2020);
    let solution = num_one * num_two * num_three;
    println!("Solution: {solution}");

}

fn merge_sort(input_vec: &Vec<i32>) -> Vec<i32> {
    if input_vec.len() <= 1 {
        input_vec.to_vec()
    } else {
        let size = input_vec.len() / 2;
        let left = merge_sort(&input_vec[..size].to_vec());
        let right = merge_sort(&input_vec[size..].to_vec());

        let mut i = 0;
        let mut j = 0;
        let mut merged: Vec<i32> = Vec::new();
        while i < left.len() && j < right.len() {
            if left[i] < right[j] {
                merged.push(left[i]);
                i = i + 1;
            } else {
                merged.push(right[j]);
                j = j + 1;
            }
        }

        while i < left.len() {
            merged.push(left[i]);
            i = i + 1;
        }

        while j < right.len() {
            merged.push(right[j]);
            j = j + 1;
        }

        merged
    }
}

fn find_nums(input_vec: &Vec<i32>, expected_sum: i32) -> (i32, i32, i32) {
    let mut i = 0;
    let mut j = input_vec.len() - 1;
    let mut k = 1;

    loop {

        if k == j {
            i = i + 1;
            k = i + 1;
            j = input_vec.len() - 1;
        }

        let sum = input_vec[i] + input_vec[k] + input_vec[j];

        if sum < expected_sum {
            k = k + 1;
        } else if sum > expected_sum {
            j = j - 1;
        } else if sum == expected_sum {
            break;
        }
    }

    (input_vec[i], input_vec[j], input_vec[k])
}