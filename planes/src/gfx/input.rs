use std::thread::{self, JoinHandle};

use crossterm::event::KeyCode;

pub fn on_input(mut press: impl FnMut(&crossterm::event::KeyEvent) + 'static + std::marker::Send) -> JoinHandle<()> {
    thread::spawn(move || loop {
        let event = crossterm::event::read().unwrap();

        if let crossterm::event::Event::Key(key_event) = event {
            if key_event.code == KeyCode::Esc {
                break;
            }

            press(&key_event);
        }
    })
}