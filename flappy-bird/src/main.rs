use std::{
    sync::{Arc, Mutex},
    thread,
};

use console::Term;
use flappy_bird::{
    exit, game,
    screen::{BgColor, FgColor, Screen, Tile},
    PIPE_GAP, PIPE_WIDTH, PLAYER_SPAWN_X, RENDER_TIME, TICK_TIME,
};

fn main() {
    let term = Term::stdout();
    let mut screen = Screen::new(&term);
    let game = game::Game::new(&screen);

    let game = Arc::new(Mutex::new(game));

    term.hide_cursor().unwrap();

    // Renders the game every 10 ms.
    let game_mutex = Arc::clone(&game);
    let render_thread = thread::spawn(move || loop {
        thread::sleep(RENDER_TIME);

        let game = game_mutex.lock().unwrap();

        screen.reset();

        for pipe in &game.pipes {
            screen.rectangle(
                pipe.pos_x,
                0,
                pipe.pos_x + PIPE_WIDTH,
                pipe.offset_y - PIPE_GAP,
                Tile::new(FgColor::Black, BgColor::Green, b'@'),
            );

            screen.rectangle(
                pipe.pos_x,
                pipe.offset_y + PIPE_GAP,
                pipe.pos_x + PIPE_WIDTH,
                screen.height,
                Tile::new(FgColor::Black, BgColor::Green, b'@'),
            );

            // screen.set(pipe.pos_x, pipe.offset_y, Tile::new(FgColor::Black, BgColor::Red, b'0'));
            // screen.set(pipe.pos_x, pipe.offset_y+PIPE_GAP, Tile::new(FgColor::Black, BgColor::Black, b'+'));
            // screen.set(pipe.pos_x, pipe.offset_y-PIPE_GAP-1, Tile::new(FgColor::Black, BgColor::Yellow, b'-'));
        }

        screen.set(
            PLAYER_SPAWN_X - 2,
            game.player_y,
            Tile::new(FgColor::Black, BgColor::Yellow, b'#'),
        );
        screen.set(
            PLAYER_SPAWN_X - 1,
            game.player_y,
            Tile::new(FgColor::Black, BgColor::Yellow, b'#'),
        );
        screen.set(
            PLAYER_SPAWN_X,
            game.player_y,
            Tile::new(FgColor::Black, BgColor::Red, b'>'),
        );

        screen.render(&format!("Score: {}", game.score));
    });

    // Gathers input in a seperate thread.
    let game_mutex = Arc::clone(&game);
    let input_thread = thread::spawn(move || loop {
        let key = term
            .read_key()
            .expect("an error occurred while reading input");

        let mut game = game_mutex.lock().unwrap();

        match key {
            console::Key::ArrowUp => game.flap(),
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
