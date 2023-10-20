use std::{fs, sync::{Mutex, Arc}, thread, time::Duration};

use crossterm::event::KeyCode;
use invaders::{gfx::{self, screen::{Screen, RGB}, font::Font, input::on_input}, game::{Game, Bullet}};

fn main() {
    let screen = Screen::new(|screen| {
        screen.bg(RGB(0, 0, 0));
    }, "Space Invaders!");

    let game = Arc::new(Mutex::new(Game::init(screen.width, screen.height)));

    let bottom = screen.height - 16;

    let sprites = Screen::load_image("./art/invaders.png");
    let invader_sprite = Screen::load_section(&sprites, 3, 4, 14, 12);
    let ship_sprite = Screen::load_section(&sprites, 64, 0, 80, 16);
    let font = Font::load(&fs::read("./font/font9.psfu").unwrap()).unwrap();

    let game_mutex = game.clone();
    let (sender, render) = screen.on_update(move |screen| {
        let game = game_mutex.lock().unwrap();

        screen.text(0, 0, RGB(255, 255, 255), &font, &format!("Score: {}", game.score));

        for bullet in &game.bullets {
            screen.set_pixel(bullet.x, bullet.y, RGB(255, 255, 255))
        }

        for invader_row in &game.invaders_group.invaders {
            for invader in invader_row {
                screen.image(invader.x+game.invaders_group.x, invader.y+game.invaders_group.y, &invader_sprite, false, false, false)
            }
        }

        screen.image(game.x, bottom, &ship_sprite, false, false, false);
    });

    let game_mutex = game.clone();
    let input = on_input(move |key| {
        let mut game = game_mutex.lock().unwrap();
        match key.code {
            KeyCode::Right => game.x += 1,
            KeyCode::Left => game.x -= 1,
            KeyCode::Enter => {
                let x = game.x + 8;
                
                game.bullets.push(Bullet::new(x, bottom + 4))
            },
            _ => ()
        };
    });

    let game_mutex = game.clone();
    loop {
        if input.is_finished() {
            break;
        }

        thread::sleep(Duration::from_millis(16));

        // Written to be able to switch to `try_lock` if needed.
        if let Ok(mut game) = game_mutex.lock() {
            game.tick();
        }
    }

    input.join().unwrap();
    sender.send(gfx::screen::ChannelMessage::Stop).unwrap();
    render.join().unwrap();
}
