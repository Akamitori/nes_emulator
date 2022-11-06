use crate::roms::runner;
pub fn run() {
    let game_code=std::fs::read("snake.nes").unwrap();
    runner::run(game_code);
}