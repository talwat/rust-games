use std::{
    fs,
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration, process::exit,
};

use crossterm::{event::KeyCode, execute};
use invaders::{
    game::{Bullet, Game, StateMachine, DEFAULT_MENU},
    gfx::{
        self,
        input::on_input,
        screen::{Screen, RGB},
    },
};
use psf_rs::Font;

fn main() {
    execute!(
        io::stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )
    .unwrap();

    let term = crossterm::terminal::size().unwrap();

    if !(term.0 >= 148 && term.1 >= 64) {
        print!("Terminal is too small! Current: {}x{}, Needed: 148x64", term.0, term.1);

        exit(1);
    }

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
    let font = Font::load(&fs::read("./font/font9.psfu").unwrap());
    let font_big = Font::load(&fs::read("./font/font16.psfu").unwrap());

    let mut flip_flop_timer = 16;

    let game_mutex = game.clone();
    let (sender, render) = screen.on_update(move |screen| {
        if flip_flop_timer == 0 {
            flip_flop_timer = 16
        } else {
            flip_flop_timer -= 1;
        }

        let mut game = game_mutex.lock().unwrap();

        if game.state == StateMachine::Credits {
            screen.text(
                4,
                4,
                RGB(255, 255, 255),
                &font,
                "Made By:
Talwat

Libraries:
- crossterm
- rand
- image",
            );

            return;
        }

        if let StateMachine::Menu(menu) = &game.state {
            let title = "Space Invaders!";
            let spacing = (screen.width - (font_big.header.glyph_width as usize * title.len())) / 2;

            screen.text(
                spacing,
                (screen.height - font_big.header.glyph_height as usize) / 2 - 32,
                RGB(255, 255, 255),
                &font_big,
                title,
            );

            for (i, option) in menu.options.iter().enumerate() {
                screen.text(
                    spacing,
                    ((screen.height - font.header.glyph_height as usize) / 2) + (i * 12),
                    RGB(255, 255, 255),
                    &font,
                    option.to_str(),
                );

                if i == menu.cursor_index {
                    screen.text(
                        spacing - 12,
                        ((screen.height - font.header.glyph_height as usize) / 2) + (i * 12),
                        RGB(255, 255, 255),
                        &font,
                        ">",
                    );
                }
            }

            // TODO: Add menu and shit.
            // TODO: Also implement changing state from menu, and add constant to be able to go back to the menu.

            return;
        }

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
            &format!("Lives {} Timer: {}", game.lives, game.invincible_timer),
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
                    None,
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
                    None,
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
                    None,
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
                None,
            );

            if explosion.timer == 0 {
                explosion.timer = 8;
                explosion.stage -= 1;
            } else {
                explosion.timer -= 1;
            }

            explosion.stage != 0
        });

        if game.invincible_timer % 2 == 0 {
            screen.image(
                game.ship.x,
                game.ship.y,
                &ship_sprite,
                false,
                false,
                false,
                None,
            );
        }
    });

    let game_mutex = game.clone();
    let input = on_input(move |key| {
        let mut game = game_mutex.lock().unwrap();
        match &mut game.state {
            StateMachine::Play => match key.code {
                KeyCode::Right | KeyCode::Char('d') => game.ship.x += 2,
                KeyCode::Left | KeyCode::Char('w') => game.ship.x -= 2,
                KeyCode::Enter | KeyCode::Char(' ') => {
                    let x = game.ship.x + 4;
                    let y = game.ship.y + 4;

                    game.bullets.push(Bullet::new(x, y, false))
                }
                _ => (),
            },
            StateMachine::Loss | StateMachine::Win => match key.code {
                _ => *game = Game::init(game.width, game.height),
            },
            StateMachine::Credits => match key.code {
                _ => game.state = StateMachine::Menu(DEFAULT_MENU),
            },
            StateMachine::Menu(menu) => match key.code {
                KeyCode::Up => {
                    if menu.cursor_index > 0 {
                        menu.cursor_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if menu.cursor_index < menu.options.len() - 1 {
                        menu.cursor_index += 1;
                    }
                }
                KeyCode::Enter => match menu.options[menu.cursor_index] {
                    invaders::game::MenuOption::Play => game.state = StateMachine::Play,
                    invaders::game::MenuOption::Credits => game.state = StateMachine::Credits,
                },
                _ => (),
            },
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
