use std::{
    io::{stdin, Read},
    sync::mpsc::channel,
    thread, time::{self, Duration},
};

use crossterm::execute;
use planes::gfx;

fn main() {
    print!("\x1b[?25l");

    let (sender, receiver) = channel::<gfx::ChannelMessage>();

    let screen = gfx::Screen::new(|screen| {
        screen.bg(gfx::RGB(135, 206, 235));
    });

    let mut i = 0;
    let plane = gfx::Screen::load_image("./small.png");

    let update = screen.update(receiver, move |screen| {
        i = i + 1;
        
        if i > screen.width - plane[0].len() {
            i = 0;
        }

        // screen.rectangle(i, 0, i+2, 10, gfx::RGB(0, 255, 0));
        // screen.circle(40, 20, 10, gfx::RGB(0, 0, 255));
        screen.image(i, 20, &plane, false, true);
    });

    stdin().read(&mut [0]).unwrap();
    sender.send(gfx::ChannelMessage::Stop).unwrap();

    print!("\x1b[2J\x1b[H\x1bc\x1b[?25h");
}
