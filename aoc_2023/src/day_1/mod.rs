/*
https://adventofcode.com/2023/day/1

The newly-improved calibration document consists of lines of text;
each line originally contained a specific calibration value that the Elves now need to recover.
On each line, the calibration value can be found by combining
the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77.
Adding these together produces 142.
 */

use regex::Regex;
use std::fs;
use std::vec::IntoIter;

const ARRAY_DOUBLES: [(&str, &str); 4] = [
    ("oneight", "18"),
    ("eightwo", "82"),
    ("eighthree", "83"),
    ("sevenine", "79"),
];

const ARRAY_FROM_LETTERS_TO_INT: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn read_file(file_path: &str) -> IntoIter<String> {
    let content = fs::read_to_string(file_path).expect("Could not read");
    content
        .split("\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
        .into_iter()
}

pub fn run_day_1(file_name: &str) -> i32 {
    // https://docs.rs/regex/1.10.2/regex/struct.Captures.html

    let lines = read_file(format!("src/day_1/{}", file_name).as_str());
    //let lines = read_file("src/day_1/exemple_bonus.txt");
    //let lines = read_file("src/day_1/exemple.txt");
    let re: Regex = Regex::new(r"^[^\d]*(\d){0,1}.*(\d)[^\d]*$").unwrap();

    let mut sum = 0;
    for (i, line) in lines.enumerate() {
        print!("{}, '{}' / ", i, String::from(line.clone()));
        let mut new_line = String::from(line);
        new_line = replace_letters_to_digits(new_line, ARRAY_DOUBLES.to_vec());
        new_line = replace_letters_to_digits(new_line, ARRAY_FROM_LETTERS_TO_INT.to_vec());

        let i1 = count_int_line(&re, &new_line);
        sum += i1;
        println!("'{}' => {} (sum = {})", new_line, i1, sum)
    }

    println!("{}", sum);
    return sum;
}

fn count_int_line(re: &Regex, new_line: &String) -> i32 {
    if let Some(table) = re.captures(&new_line) {
        let table_values = table
            .iter()
            .filter(|c| c.is_some())
            .skip(1)
            .map(|c| c.unwrap().as_str())
            .collect::<Vec<&str>>();

        if table_values.is_empty() || table_values.len() > 2 {
            return 0;
        }

        return [
            *table_values.first().unwrap(),
            *table_values.last().unwrap(),
        ]
        .join("")
        .parse::<i32>()
        .expect(format!("Could not parse {:?}", table_values).as_str());
    }
    0
}

fn replace_letters_to_digits(line: String, array_of_tuples: Vec<(&str, &str)>) -> String {
    let mut new_line = String::from(line);
    let mut indexes = vec![];

    for (index_in_array, (digit, _)) in array_of_tuples.clone().iter().enumerate() {
        if let Some(index) = new_line.find(digit) {
            indexes.push((index, index_in_array));

            if let Some(index_second) = new_line.rfind(digit) {
                if index_second != index {
                    indexes.push((index_second, index_in_array));
                }
            }
        }
    }

    if !indexes.is_empty() {
        indexes.sort_by_key(|&k| k.0);

        if indexes.len() == 1 {
            replace_in_line(&mut new_line, indexes.first(), array_of_tuples.clone());
        } else if indexes.len() > 1 {
            if !start_with_digits(&new_line.chars().rev().collect::<String>()) {
                replace_in_line(&mut new_line, indexes.last(), array_of_tuples.clone());
            }

            if !start_with_digits(&new_line) {
                let (index_in_str, index_in_array) = indexes.first().unwrap();

                let tuple: (&str, &str) = array_of_tuples[*index_in_array];
                if new_line.find(tuple.0).is_some_and(|x| x == *index_in_str) {
                    new_line.replace_range(index_in_str..&(tuple.0.len() + index_in_str), tuple.1);
                }
            }
        }
    }
    new_line
}

fn replace_in_line(
    new_line: &mut String,
    indexes: Option<&(usize, usize)>,
    array_of_tuples: Vec<(&str, &str)>,
) {
    let (index_in_str, index_in_array) = indexes.unwrap();

    let tuple: (&str, &str) = array_of_tuples[*index_in_array];
    new_line.replace_range(index_in_str..&(tuple.0.len() + index_in_str), tuple.1);
}

fn start_with_digits(new_line: &String) -> bool {
    if let Some(c) = new_line.get(..1) {
        return c.parse::<i32>().is_ok();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::{
        count_int_line, replace_letters_to_digits, run_day_1, start_with_digits, ARRAY_DOUBLES,
        ARRAY_FROM_LETTERS_TO_INT,
    };
    use regex::Regex;

    #[test]
    fn test_day_one() {
        assert_eq!(run_day_1("exemple.txt"), 142);
        assert_eq!(run_day_1("exemple_bonus.txt"), 281);
    }

    #[test]
    fn test_start_with_digits() {
        assert!(start_with_digits(&"123".to_string()));
        assert!(start_with_digits(&"1".to_string()));
        assert!(!start_with_digits(&"ab3".to_string()))
    }

    #[test]
    fn replace_letters_to_digits_test() {
        let array = ARRAY_FROM_LETTERS_TO_INT.to_vec();
        assert_eq!(
            replace_letters_to_digits("five712".to_string(), array.clone()),
            "5712"
        );
        assert_eq!(
            replace_letters_to_digits(
                "tzheightzlzmsqlnxfqzrlbhbdpbnbdjns6".to_string(),
                array.clone()
            ),
            "tzh8zlzmsqlnxfqzrlbhbdpbnbdjns6"
        );
        assert_eq!(
            replace_letters_to_digits("57eight9fivefiveeight".to_string(), array.clone()),
            "57eight9fivefive8"
        );
        assert_eq!(
            replace_letters_to_digits("52fourfour7sxzptgtnhsfour82".to_string(), array.clone()),
            "52fourfour7sxzptgtnhsfour82"
        );
        assert_eq!(
            replace_letters_to_digits("nine8bgh921seven".to_string(), array.clone()),
            "98bgh9217"
        );
        assert_eq!(
            replace_letters_to_digits("five6fivethree2three".to_string(), array.clone()),
            "56fivethree23"
        );
        assert_eq!(
            replace_letters_to_digits("seven22".to_string(), array.clone()),
            "722"
        );
        assert_eq!(
            replace_letters_to_digits("7pqrstsixteen".to_string(), array.clone()),
            "7pqrst6teen"
        );
    }

    #[test]
    fn count_in_line_test() {
        let re: Regex = Regex::new(r"^[^\d]*(\d){0,1}.*(\d)[^\d]*$").unwrap();

        let to_test = vec![
            ("8blsrrqrjlckv7xszllqddzn5oneightfg", 88),
            ("five712", 52),
            ("sstcrnkkbzfsfcnkone8", 18),
            ("fllgbdnbsztbfnjmhone7five", 15),
            ("ninejbjonexkfcm13", 93),
            ("2974rbfourfour2", 22),
            ("m4", 44),
            ("djtkclphr4ninesixfiveqgksrzj6nineeightwogz", 42),
            ("29one7672", 22),
            ("jeightwo5", 85),
            ("3ninegcnrtwotljnbkkftfourthreefour", 34),
            ("1oneeighttwo", 12),
            ("six8six", 66),
            ("3311dczzvrskxksevenpgxthreeeight", 38),
            ("p2", 22),
            ("sevensixmczghz299six", 76),
            ("threenine89zlmh9fourff", 34),
            ("eightksevenchjgvctdkfbhrxssfzcgssix21oneightrt", 88),
            ("864dvsfvcvhtrqzgspsbvgvvmpgjsppsvsbxrr5", 85),
            ("2jf", 22),
            ("eightrmkhpvkdhd3four2twoseven", 87),
            ("honeight65five3", 13),
            ("fiveeight792eightqskstrftdpccsrgskrhc", 58),
            ("rlnthmmdfsvmdfqhfivendptjfpx7hnbnkzlpntvglvdlfb4five", 55),
            ("qvggksmf1", 11),
            ("mnfhgnrlfzfive1", 51),
            ("1seventwoseven", 17),
        ];

        let array = ARRAY_FROM_LETTERS_TO_INT.to_vec();
        for (from, to) in to_test {
            assert_eq!(
                count_int_line(
                    &re,
                    &replace_letters_to_digits(from.to_string(), array.clone())
                ),
                to
            );
        }

        for (from, to) in [("eighthree", 83), ("sevenine", 79)] {
            assert_eq!(
                count_int_line(
                    &re,
                    &replace_letters_to_digits(from.to_string(), ARRAY_DOUBLES.clone().to_vec())
                ),
                to
            );
        }
    }
}
