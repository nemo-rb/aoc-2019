#[macro_use] extern crate log;


mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;


pub fn run() {
    info!("Advent of code");

    let day_one_result = day_one::run().unwrap();
    info!("1-2 result {}", day_one_result);
    let day_two_result_1 = day_two::run_1().unwrap();
    info!("2-1 result {}", day_two_result_1);
    let day_two_result_2 = day_two::run_2().unwrap();
    info!("2-2 result {}", day_two_result_2);
    let day_three_result_1 = day_three::run_1().unwrap();
    info!("3-1 result {}", day_three_result_1);
    let day_three_result_2 = day_three::run_2().unwrap();
    info!("3-2 result {}", day_three_result_2);
    let day_four_result_1 = day_four::run_1();
    info!("4-1 result {}", day_four_result_1);
    let day_four_result_2 = day_four::run_2();
    info!("4-2 result {}", day_four_result_2);
    let day_five_result_1 = day_five::run_1().unwrap();
    info!("5-1 result {}", day_five_result_1);
}
