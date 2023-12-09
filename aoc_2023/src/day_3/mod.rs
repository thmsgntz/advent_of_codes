use std::cmp::{max, min};
use std::fs;

pub(crate) fn run_day_3(file_path: &str) -> i32 {
    let lines = fs::read_to_string(format!("src/day_3/{}", file_path))
        .expect("Should be able to read this");

    let mut sum = 0;
    let mat = to_matrix(lines);
    for i in 0..mat.len() {
        println!("Line {} => {:?}", i, mat[i]);

        let mat_line = &mat[i];
        let mut j = 0;
        while j < mat_line.len() {
            if is_digit(&mat_line[j]) {
                let (number, len_number) = get_number_from_vec_of_str(&mat_line[j..]);
                print!("Found number {} at ({}, {}) ..", number, i, j);
                let top_left_i = max(i as i32 - 1, 0) as usize;
                let top_left_j = max(j as i32 - 1, 0) as usize;
                let bot_right_i = min(i as i32 + 1, mat.len() as i32 - 1) as usize;
                let bot_right_j =
                    min(j as i32 + len_number as i32, mat_line.len() as i32 - 1) as usize;

                println!(
                    "Will scan between ({},{}) to ({},{})",
                    top_left_i, top_left_j, bot_right_i, bot_right_j
                );

                if is_symbole_in_mat(&mat, top_left_i, top_left_j, bot_right_i, bot_right_j) {
                    sum += number;
                    println!("=> True! Adding {}", number);
                }

                // find bot_right_corner = len_number+i, j+1
                // extract matrice[i-1, j-1:len_number+i, j+1]
                // loop over all elements, return true is symbol, else false
                // add it if true
                j += len_number;
            } else {
                j += 1;
            }
        }
        println!();
    }
    println!("Final sum: {}", sum);
    return sum;
}

fn is_symbole_in_mat(
    mat: &Vec<Vec<String>>,
    top_left_i: usize,
    top_left_j: usize,
    bot_right_i: usize,
    bot_right_j: usize,
) -> bool {
    for i in top_left_i..bot_right_i + 1 {
        for j in top_left_j..bot_right_j + 1 {
            if is_symbol(mat[i][j].as_str()) {
                return true;
            }
        }
    }
    return false;
}

fn to_matrix(lines_from_file: String) -> Vec<Vec<String>> {
    lines_from_file
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            String::from(s)
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}

fn is_digit(one_char_as_str: &str) -> bool {
    one_char_as_str.parse::<usize>().is_ok()
}

fn is_symbol(one_char_as_str: &str) -> bool {
    !is_digit(one_char_as_str) && one_char_as_str != "."
}

/// From ["1", "2", "3"], return 123
/// First index must be a digit, or panic.
fn get_number_from_vec_of_str(v: &[String]) -> (i32, usize) {
    let mut i = 0;
    while i < v.len() && is_digit(v[i].as_str()) {
        i += 1;
    }
    (v[0..i].join("").parse::<i32>().unwrap(), i)
}

#[cfg(test)]
mod test {
    use super::{get_number_from_vec_of_str, is_symbol, run_day_3};

    #[test]
    fn test_run_example() {
        assert_eq!(run_day_3("example.txt"), 4361);
    }

    #[test]
    fn test_get_number() {
        let v = vec!["1", "2", "3", "4", "5", ".", ".", "7", "8", "."]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(get_number_from_vec_of_str(&v[1..]), (2345, 4));
        assert_eq!(get_number_from_vec_of_str(&v[2..5]), (345, 3));
        assert_eq!(get_number_from_vec_of_str(&v[7..9]), (78, 2));
        assert_eq!(get_number_from_vec_of_str(&v[8..9]), (8, 1));
    }

    #[test]
    fn test_is_symbole() {
        assert!(!is_symbol("."));
        assert!(!is_symbol("1"));
        assert!(is_symbol("*"));
        assert!(is_symbol("-"));
        assert!(is_symbol("@"));
        assert!(is_symbol("#"));
        assert!(is_symbol("/"));
        assert!(is_symbol("$"));
        assert!(is_symbol("="));
    }
}
