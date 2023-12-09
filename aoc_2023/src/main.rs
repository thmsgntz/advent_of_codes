mod day_1;
mod day_2;
mod day_3;
mod day_4;

use std::env;

fn main() {
    for arg in env::args() {
        match arg.as_str() {
            "1" => {
                day_1::run_day_1("input.txt");
            }
            "2" => {
                day_2::run_day_2("input.txt", 12, 14, 13);
                day_2::run_day_2_bonus("input.txt");
            }
            "3" => {
                day_3::run_day_3("input.txt");
            }
            "4" => {
                day_4::run_day_4("input.txt");
                day_4::run_day_4_bonus("input.txt", 223);
            }
            _ => {
                eprintln!("Wrong input");
            }
        }
    }
}
