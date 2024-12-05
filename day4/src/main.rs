// https://adventofcode.com/2024/day/4

use utils::get_csv_data;

fn get_word_search(path: &str) -> Vec<Vec<String>> {
    let mut word_search_matrix: Vec<Vec<String>> = get_csv_data(path, false);

    for i in 0..word_search_matrix.len() {
        word_search_matrix[i] = word_search_matrix[i][0].split("").map(String::from).collect();
    }

    return word_search_matrix;
}

fn horizontal_search(word_matrix: &[Vec<String>]) -> u32 {
    // -
    let mut count: u32 = 0;

    for i in 0..word_matrix.len() {
        for j in 0..(word_matrix[i].len() - 3) {
            let possible_word: Vec<String> = vec![
                word_matrix[i][j].clone(),
                word_matrix[i][j + 1].clone(),
                word_matrix[i][j + 2].clone(),
                word_matrix[i][j + 3].clone(),
            ];
            if possible_word == vec!["X", "M", "A", "S"] || possible_word == vec!["S", "A", "M", "X"] {
                count += 1;
            }
        }
    }

    return count;
}

fn virtical_search(word_matrix: &[Vec<String>]) -> u32 {
    // |
    let mut count: u32 = 0;

    for i in 0..(word_matrix.len() - 3) {
        for j in 0..word_matrix[i].len() {
            let possible_word: Vec<String> = vec![
                word_matrix[i][j].clone(),
                word_matrix[i + 1][j].clone(),
                word_matrix[i + 2][j].clone(),
                word_matrix[i + 3][j].clone(),
            ];
            if possible_word == vec!["X", "M", "A", "S"] || possible_word == vec!["S", "A", "M", "X"] {
                count += 1;
            }
        }
    }

    return count;
}

fn diagnal_right_search(word_matrix: &[Vec<String>]) -> u32 {
    // \
    let mut count: u32 = 0;

    for i in 0..(word_matrix.len() - 3) {
        for j in 0..(word_matrix[i].len() - 3) {
            let possible_word: Vec<String> = vec![
                word_matrix[i][j].clone(),
                word_matrix[i + 1][j + 1].clone(),
                word_matrix[i + 2][j + 2].clone(),
                word_matrix[i + 3][j + 3].clone(),
            ];
            if possible_word == vec!["X", "M", "A", "S"] || possible_word == vec!["S", "A", "M", "X"] {
                count += 1;
            }
        }
    }

    return count;
}

fn diagnal_left_search(word_matrix: &[Vec<String>]) -> u32 {
    // /
    let mut count: u32 = 0;

    for i in 0..(word_matrix.len() - 3) {
        for j in 3..word_matrix[i].len() {
            let possible_word: Vec<String> = vec![
                word_matrix[i][j].clone(),
                word_matrix[i + 1][j - 1].clone(),
                word_matrix[i + 2][j - 2].clone(),
                word_matrix[i + 3][j - 3].clone(),
            ];
            if possible_word == vec!["X", "M", "A", "S"] || possible_word == vec!["S", "A", "M", "X"] {
                count += 1;
            }
        }
    }

    return count;
}

pub fn word_search(word_matrix: &[Vec<String>]) -> u32 {
    let mut count: u32 = 0;
    count += horizontal_search(word_matrix);
    count += virtical_search(word_matrix);
    count += diagnal_right_search(word_matrix);
    count += diagnal_left_search(word_matrix);

    return count;
}

pub fn x_mas_search(word_matrix: &[Vec<String>]) -> u32 {
    let mut count: u32 = 0;

    for i in 0..(word_matrix.len() - 2) {
        for j in 0..(word_matrix[i].len() - 2) {
            let possible_x: Vec<Vec<String>> = vec![
                vec![
                    word_matrix[i][j].clone(),
                    word_matrix[i][j + 1].clone(),
                    word_matrix[i][j + 2].clone(),
                ],
                vec![
                    word_matrix[i + 1][j].clone(),
                    word_matrix[i + 1][j + 1].clone(),
                    word_matrix[i + 1][j + 2].clone(),
                ],
                vec![
                    word_matrix[i + 2][j].clone(),
                    word_matrix[i + 2][j + 1].clone(),
                    word_matrix[i + 2][j + 2].clone(),
                ],
            ];

            if possible_x[1][1] == "A"
                && ((possible_x[0][0] == "M" && possible_x[2][2] == "S")
                    || (possible_x[0][0] == "S" && possible_x[2][2] == "M"))
                && ((possible_x[2][0] == "M" && possible_x[0][2] == "S")
                    || (possible_x[2][0] == "S" && possible_x[0][2] == "M"))
            {
                count += 1;
            }
        }
    }

    return count;
}

fn main() {
    let word_search_matrix = get_word_search("data/input.csv");

    let word_search_count = word_search(&word_search_matrix);
    println!("word search count: {}", word_search_count);

    let x_mas_count = x_mas_search(&word_search_matrix);
    println!("x-mas count: {}", x_mas_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_search() {
        let word_search_matrix = get_word_search("data/test.csv");
        let count = word_search(&word_search_matrix);
        assert_eq!(count, 18);
    }

    #[test]
    fn test_x_mas_search() {
        let word_search_matrix = get_word_search("data/test.csv");
        let count = x_mas_search(&word_search_matrix);
        assert_eq!(count, 9);
    }
}
