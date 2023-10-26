use std::{
    fs,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossterm::event::KeyCode;
use invaders::{
    game::{Bullet, Game, StateMachine},
    gfx::{
        self,
        font::Font,
        input::on_input,
        screen::{Screen, RGB},
    },
};

fn main() {
    let screen = Screen::new(
        |screen| {
            screen.bg(RGB(0, 0, 0));
        },
        "Space Invaders!",
    );

    let game = Arc::new(Mutex::new(Game::init(screen.width, screen.height)));

    let sprites = Screen::load_image("./art/invaders.png");
    let invader_sprites = [
        Screen::load_section(&sprites, 3, 4, 14, 12), // 20
        Screen::load_section(&sprites, 3, 4 + 16, 14, 12 + 16), // 30
        Screen::load_section(&sprites, 3, 4 + 32, 14, 12 + 32), // 10
    ];
    let explosion_sprites = [
        Screen::load_section(&sprites, 32, 64, 48, 80),
        Screen::load_section(&sprites, 32, 48, 48, 64),
        Screen::load_section(&sprites, 32, 32, 48, 48),
    ];
    let wall_sprites = [
        Screen::load_section(&sprites, 51, 20 + 48, 77, 32 + 48),
        Screen::load_section(&sprites, 51, 20 + 32, 77, 32 + 32),
        Screen::load_section(&sprites, 51, 20 + 16, 77, 32 + 16),
        Screen::load_section(&sprites, 51, 20, 77, 32),
    ];
    let invader_bullet = Screen::load_section(&sprites, 37, 21, 41, 28);
    let ship_sprite = Screen::load_section(&sprites, 68, 4, 77, 14);
    let font = Font::load(&fs::read("./font/font9.psfu").unwrap()).unwrap();
    let font_big = Font::load(&fs::read("./font/font16.psfu").unwrap()).unwrap();

    let mut flip_flop_timer = 16;

    let game_mutex = game.clone();
    let (sender, render) = screen.on_update(move |screen| {
        if flip_flop_timer == 0 {
            flip_flop_timer = 16
        } else {
            flip_flop_timer -= 1;
        }

        let mut game = game_mutex.lock().unwrap();

        if game.state == StateMachine::Win || game.state == StateMachine::Loss {
            let score_text = &format!("Score: {}", game.score);
            let big_text = if game.state == StateMachine::Win {
                "You Win! "
            } else {
                "You Lose!"
            };

            screen.text(
                (screen.width - (font_big.header.glyph_width as usize * big_text.len())) / 2,
                (screen.height - font_big.header.glyph_height as usize) / 2 - 16,
                RGB(255, 255, 255),
                &font_big,
                big_text,
            );
            screen.text(
                (screen.width - (font.header.glyph_width as usize * score_text.len())) / 2,
                (screen.height - font.header.glyph_height as usize) / 2,
                RGB(255, 255, 255),
                &font,
                score_text,
            );
            screen.text(
                (screen.width - (font.header.glyph_width as usize * 7)) / 2,
                ((screen.height - font.header.glyph_height as usize) / 2) + 8,
                RGB(255, 255, 255),
                &font,
                "Made By",
            );
            screen.text(
                (screen.width - (font.header.glyph_width as usize * 6)) / 2,
                ((screen.height - font.header.glyph_height as usize) / 2) + 16,
                RGB(255, 255, 255),
                &font,
                "Talwat",
            );

            return;
        }

        screen.text(
            0,
            0,
            RGB(255, 255, 255),
            &font,
            &format!("Score {}", game.score),
        );

        screen.text(
            0,
            screen.height - 9,
            RGB(255, 255, 255),
            &font,
            &format!("Lives {}", game.lives),
        );

        for bullet in &game.bullets {
            if bullet.invader {
                screen.image(
                    bullet.transform.x - 2,
                    bullet.transform.y - 3,
                    &invader_bullet,
                    flip_flop_timer >= 8,
                    false,
                    false,
                );
            } else {
                screen.set_pixel(bullet.transform.x, bullet.transform.y, RGB(255, 255, 255))
            }
        }

        for invader_row in &game.invaders_group.invaders {
            for invader in invader_row {
                let sprite = match invader.score {
                    30 => &invader_sprites[1],
                    20 => &invader_sprites[0],
                    10 => &invader_sprites[2],
                    _ => &invader_sprites[0],
                };

                screen.image(
                    invader.transform.x + game.invaders_group.x,
                    invader.transform.y + game.invaders_group.y,
                    sprite,
                    false,
                    false,
                    false,
                )
            }
        }

        for wall in game.walls {
            if wall.health > 0 {
                screen.image(
                    wall.transform.x,
                    wall.transform.y,
                    &wall_sprites[wall.health as usize - 1],
                    false,
                    false,
                    false,
                );
            }
        }

        game.effects.explosions.retain_mut(|explosion| {
            screen.image(
                explosion.x - 8,
                explosion.y - 12,
                &explosion_sprites[explosion.stage - 1],
                false,
                false,
                false,
            );

            if explosion.timer == 0 {
                explosion.timer = 8;
                explosion.stage -= 1;
            } else {
                explosion.timer -= 1;
            }

            explosion.stage != 0
        });

        screen.image(game.ship.x, game.ship.y, &ship_sprite, false, false, false);
    });

    let game_mutex = game.clone();
    let input = on_input(move |key| {
        let mut game = game_mutex.lock().unwrap();
        match key.code {
            KeyCode::Right => game.ship.x += 2,
            KeyCode::Left => game.ship.x -= 2,
            KeyCode::Enter => {
                let x = game.ship.x + 4;
                let y = game.ship.y + 4;

                game.bullets.push(Bullet::new(x, y, false))
            }
            _ => (),
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
