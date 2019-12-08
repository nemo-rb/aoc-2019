#[macro_use] extern crate log;


mod day_one;


pub fn run() -> Result<(), std::io::Error> {
    info!("Advent of code");

    let day_one_result = day_one::calculate_fuel_required();
    info!("Day one result {}", day_one_result?);
    Ok(())
}
