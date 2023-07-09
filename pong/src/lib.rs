pub mod game;
pub mod screen;

use std::time::Duration;

pub const RENDER_TIME: Duration = Duration::from_millis(10);
pub const TICK_TIME: Duration = Duration::from_millis(60);

pub const PADDLE_PADDING: usize = 5;
pub const PADDLE_HEIGHT: usize = 2;

pub fn exit(exit_message: &str) {
    println!("\r\x1b[2J\r\x1b[H\x1b[?25h{exit_message}");

    std::process::exit(0)
}
