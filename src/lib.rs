mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod day_eight;



pub fn run() {
    println!("Advent of code");

    let day_one_result = day_one::run().unwrap();
    println!("1-2 result {}", day_one_result);
    let day_two_result_1 = day_two::run_1().unwrap();
    println!("2-1 result {}", day_two_result_1);
    let day_two_result_2 = day_two::run_2().unwrap();
    println!("2-2 result {}", day_two_result_2);
    let day_three_result_1 = day_three::run_1().unwrap();
    println!("3-1 result {}", day_three_result_1);
    let day_three_result_2 = day_three::run_2().unwrap();
    println!("3-2 result {}", day_three_result_2);
    let day_four_result_1 = day_four::run_1();
    println!("4-1 result {}", day_four_result_1);
    let day_four_result_2 = day_four::run_2();
    println!("4-2 result {}", day_four_result_2);
    let day_five_result_1 = day_five::run_1().unwrap();
    println!("5-1 result {}", day_five_result_1);
    let day_five_result_2 = day_five::run_2().unwrap();
    println!("5-2 result {}", day_five_result_2);
    let day_six_result_1 = day_six::run_1();
    println!("6-1 result {:?}", day_six_result_1);
    let day_six_result_2 = day_six::run_2();
    println!("6-2 result {}", day_six_result_2);
    let day_seven_result_1 = day_seven::run_1();
    println!("7-1 result {}", day_seven_result_1);
    let day_seven_result_2 = day_seven::run_2();
    println!("7-2 result {}", day_seven_result_2);
    let day_eight_result_1 = day_eight::run_1().unwrap();
    println!("8-1 result {}", day_eight_result_1);
    day_eight::run_2();
}
