use regex::Regex;
use std::fs;
use std::num::ParseIntError;

/// From "Game 85: 1 red, 2 blue, 9 green; ..." return 85
fn extract_game_number(re: Regex, line: &str) -> Result<i32, ParseIntError> {
    if let Some(ma) = re.captures(line).and_then(|ma| ma.get(1)) {
        return ma.as_str().parse::<i32>();
    } else {
        panic!("Could not find 'Game Number'")
    }
}

fn extract_number_of_color(line: &str, color: &str) -> Vec<i32> {
    let color_formatted_with_white_space = format!(" {}", color);
    let re_color =
        Regex::new(format!("(\\d*){}", color_formatted_with_white_space).as_str()).unwrap();

    re_color
        .find_iter(line)
        .map(|m| {
            m.as_str()
                .replace(color_formatted_with_white_space.as_str(), "")
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>()
}

pub(crate) fn run_day_2_bonus(input_file: &str) -> i32 {
    let lines = read_file(input_file);

    let mut sum = 0;
    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }

        let v_blues = extract_number_of_color(line, "blue");
        let v_red = extract_number_of_color(line, "red");
        let v_green = extract_number_of_color(line, "green");

        print!("B: {:?} / R: {:?} / G {:?} => ", v_blues, v_red, v_green);

        sum += v_blues.iter().max().unwrap()
            * v_red.iter().max().unwrap()
            * v_green.iter().max().unwrap();
    }
    println!("Somme; {}", sum);
    return sum;
}

pub(crate) fn run_day_2(
    input_file: &str,
    max_red: usize,
    max_blue: usize,
    max_green: usize,
) -> i32 {
    let lines = read_file(input_file);
    let re_game_number = Regex::new(r"Game (\d*):.*").expect("Game Number Regex is wrong");

    let mut sum = 0;
    for (_i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            continue;
        }

        let v_blues = extract_number_of_color(line, "blue");
        let v_red = extract_number_of_color(line, "red");
        let v_green = extract_number_of_color(line, "green");

        print!("B: {:?} / R: {:?} / G {:?} => ", v_blues, v_red, v_green);
        if v_blues.iter().max().unwrap() <= &(max_blue as i32)
            && v_green.iter().max().unwrap() <= &(max_green as i32)
            && v_red.iter().max().unwrap() <= &(max_red as i32)
        {
            let game_number = extract_game_number(re_game_number.clone(), &line)
                .expect("Could not parse game number of line");

            sum += game_number;
            println!("OK");
        } else {
            println!("NOP");
        }
    }
    println!("Somme; {}", sum);
    return sum;
}

fn read_file(input_file: &str) -> Vec<String> {
    let lines = fs::read_to_string(format!("src/day_2/{}", input_file)).expect("Mauvais chemin?");
    lines
        .split("\n")
        .into_iter()
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::{extract_game_number, extract_number_of_color, run_day_2, run_day_2_bonus};
    use regex::Regex;

    #[test]
    fn test_example() {
        assert_eq!(run_day_2("example.txt", 12, 14, 13), 8);
    }

    #[test]
    fn test_example_bonus() {
        assert_eq!(run_day_2_bonus("example.txt"), 2286);
    }

    #[test]
    fn test_extract_number_colors() {
        assert_eq!(
            extract_number_of_color("9 blue, 1 green, 20 red; 9 green, 20 red, 16 blue;", "blue"),
            vec![9, 16]
        );

        assert_eq!(
            extract_number_of_color("9 blue, 1 green, 20 red; 9 green, 20 red, 16 blue;", "red"),
            vec![20, 20]
        );

        assert_eq!(
            extract_number_of_color(
                "9 blue, 1 green, 20 red; 9 green, 20 red, 16 blue;",
                "green"
            ),
            vec![1, 9]
        );

        assert_eq!(
            extract_number_of_color("Game 78: 4 red, 8 blue, 2 green; 7 blue, 3 green, 7 red; 3 green, 13 blue; 3 red, 4 green", "blue"),
            vec![8, 7, 13]
        )
    }

    #[test]
    fn test_extract_game_number() {
        let re_game_number = Regex::new(r"e (\d*):").expect("Game Number Regex is wrong");
        assert_eq!(
            extract_game_number(re_game_number.clone(), "Game 100: 2 blue, 12 green; 6 aaa"),
            Ok(100)
        );
        assert_eq!(
            extract_game_number(re_game_number.clone(), "Game 1: 2 blue, 12 green; 6 aaa"),
            Ok(1)
        );
        assert_eq!(
            extract_game_number(re_game_number.clone(), "Game 56: 2 blue, 12 green; 6 aaa"),
            Ok(56)
        );
    }
}
