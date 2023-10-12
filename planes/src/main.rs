use std::{
    sync::{Arc, Mutex}, fs,
};

use crossterm::event::KeyCode;
use planes::gfx::{self, screen::{Screen, RGB}, font::Font, input::on_input};

fn main() {
    let screen = Screen::new(|screen| {
        screen.bg(RGB(135, 206, 235));
    });

    let pos = Arc::new(Mutex::new((0, 0)));
    let plane = Screen::load_image("./small.png");
    let font = Font::load(&fs::read("./font.psfu").unwrap()).unwrap();

    let pos_mutex = pos.clone();
    let (sender, update) = screen.on_update(move |screen| {
        let pos = *pos_mutex.lock().unwrap();

        // screen.text(20, 0, RGB(0, 0, 0), &font, "google en");
        // screen.text(20, 20, RGB(0, 0, 0), &font, "passant");
        // screen.image(pos.0, pos.1, &plane, true, false, true);
        // screen.rectangle(10, 40, 50, 50, RGB(255, 0, 0));
        // screen.circle(70, 60, 10, RGB(0, 255, 0));
        //screen.line(0, 0, screen.width-1, screen.height-1, RGB(0, 255, 0));
        screen.triangle(0, 0, 0, 10, 40, 10, RGB(0, 255, 0))
    });

    let pos_mutex = pos.clone();
    let input = on_input(move |key| {
        let mut pos = pos_mutex.lock().unwrap();

        match key.code {
            KeyCode::Right => pos.0 += 1,
            KeyCode::Left => pos.0 -= 1,
            KeyCode::Down => pos.1 += 1,
            KeyCode::Up => pos.1 -= 1,
            _ => ()
        };
    });

    input.join().unwrap();
    sender.send(gfx::screen::ChannelMessage::Stop).unwrap();
    update.join().unwrap();
}
