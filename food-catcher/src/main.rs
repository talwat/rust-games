use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;

use food_catcher::display;
use food_catcher::game;
use food_catcher::game::TickStatus;

fn exit(exit_message: &str) {
    println!("\r\x1b[2J\r\x1b[H\x1b[?25h{exit_message}");

    std::process::exit(0)
}

fn main() {
    println!("Loading...");

    let term = console::Term::stdout();
    let mut screen = display::Screen::new(&term);
    let game = Arc::new(Mutex::new(game::Game::new(&screen)));

    term.hide_cursor().unwrap();

    let game_mutex = Arc::clone(&game);

    // Renders the game every 10 ms.
    let render_thread = thread::spawn(move || loop {
        thread::sleep(time::Duration::from_millis(10));

        let mut game = game_mutex.lock().unwrap();
        screen.render(&game);

        let status = game.tick();

        if status == TickStatus::Exit {
            exit("Game Over!")
        }
    });

    let game_mutex = Arc::clone(&game);

    // Gathers input in a seperate thread.
    let input_thread: thread::JoinHandle<()> = thread::spawn(move || loop {
        let key = term
            .read_key()
            .expect("an error occurred while reading input");

        let mut game = game_mutex.lock().unwrap();

        match key {
            console::Key::ArrowLeft => game.player_x -= 1,
            console::Key::ArrowRight => game.player_x += 1,
            console::Key::Char('q') => {
                exit("Quit.")
            }
            _ => (),
        }
    });

    render_thread.join().unwrap();
    input_thread.join().unwrap();
}
