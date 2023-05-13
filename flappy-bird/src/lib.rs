pub mod screen;
pub mod tile;
pub mod game;

use std::time::Duration;

pub const PIPE_GAP: usize = 4;
pub const PIPE_WIDTH: usize = 2;
pub const PLAYER_SPAWN_X: usize = 5;

pub const RENDER_TIME: Duration = Duration::from_millis(10);
pub const TICK_TIME: Duration = Duration::from_millis(100);

pub fn exit(exit_message: &str) {
    println!("\r\x1b[2J\r\x1b[H\x1b[?25h{exit_message}");

    std::process::exit(0)
}