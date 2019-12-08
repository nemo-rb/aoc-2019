fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    aoc_2019::run().expect("Failed to run Advnt Of Code");
    Ok(())
}
