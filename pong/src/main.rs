use std::{
    sync::{Arc, Mutex},
    thread,
};

use console::Term;
use pong::{
    exit, game,
    screen::{BgColor, FgColor, Screen, Tile},
    PADDLE_HEIGHT, PADDLE_PADDING, RENDER_TIME, TICK_TIME,
};

fn main() {
    let term = Term::stdout();
    let mut screen = Screen::new(&term);
    let mut game = game::Game::new(&screen);

    game.paddle_1_y = screen.height / 2;
    game.paddle_2_y = screen.height / 2;
    game.ball_initial_pos();

    let game = Arc::new(Mutex::new(game));

    term.hide_cursor().unwrap();

    // Renders the game every 10 ms.
    let game_mutex = Arc::clone(&game);
    let render_thread = thread::spawn(move || loop {
        thread::sleep(RENDER_TIME);

        screen.reset();

        let game = game_mutex.lock().unwrap();

        screen.rectangle(
            PADDLE_PADDING,
            game.paddle_1_y - PADDLE_HEIGHT,
            PADDLE_PADDING + 1,
            game.paddle_1_y + PADDLE_HEIGHT + 1,
            Tile::new(FgColor::Default, BgColor::Red, b'|'),
        );
        screen.rectangle(
            game.width - PADDLE_PADDING - 1,
            game.paddle_2_y - PADDLE_HEIGHT,
            game.width - PADDLE_PADDING,
            game.paddle_2_y + PADDLE_HEIGHT + 1,
            Tile::new(FgColor::Default, BgColor::Blue, b'|'),
        );

        // TODO: Rounding's a bit weird, so maybe consider changing this later.
        screen.set(
            game.ball_x.round() as usize,
            game.ball_y.round() as usize,
            Tile::new(FgColor::Default, BgColor::Default, b'o'),
        );

        let player_1_score_display = format!(
            "Angle: {}, Ball X: {}, Player 1 score: {}",
            game.ball_dir, game.ball_x, game.score_1
        );
        let player_2_score_display = format!("Player 2 score: {}", game.score_2);

        screen.render(&format!(
            "{}{}{}",
            player_1_score_display,
            " ".repeat(game.width - player_1_score_display.len() - player_2_score_display.len()),
            player_2_score_display
        ));
    });

    // Gathers input in a seperate thread.
    let game_mutex = Arc::clone(&game);
    let input_thread = thread::spawn(move || loop {
        let key = term
            .read_key()
            .expect("an error occurred while reading input");

        let mut game = game_mutex.lock().unwrap();

        match key {
            console::Key::Char('w') => game.move_paddle_1(-1),
            console::Key::Char('s') => game.move_paddle_1(1),
            console::Key::ArrowUp => game.move_paddle_2(-1),
            console::Key::ArrowDown => game.move_paddle_2(1),
            console::Key::Char('q') => exit("Quit."),
            _ => (),
        }
    });

    // The game thread, it ticks every 100 ms.
    let game_mutex = Arc::clone(&game);
    let game_thread = thread::spawn(move || loop {
        thread::sleep(TICK_TIME);

        let mut game = game_mutex.lock().unwrap();

        game.tick()
    });

    render_thread.join().unwrap();
    input_thread.join().unwrap();
    game_thread.join().unwrap();
}
