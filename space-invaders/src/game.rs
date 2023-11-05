use std::io;

use rand::Rng;

const TICKS_TO_MOVE_INVADERS: usize = 12;
const INVADER_PADDING: usize = 8;
const TICKS_TO_INVINCIBILITY: usize = 8;
pub const DEFAULT_MENU: MenuData = MenuData {
    options: [MenuOption::Play, MenuOption::Credits],
    cursor_index: 0,
};

#[derive(PartialEq)]
pub enum MenuOption {
    Play,
    Credits,
}

impl MenuOption {
    pub fn to_str(&self) -> &'static str {
        match self {
            MenuOption::Play => "Play",
            MenuOption::Credits => "Credits",
        }
    }
}
#[derive(PartialEq)]
pub struct MenuData {
    pub options: [MenuOption; 2],
    pub cursor_index: usize,
}

/// The different states the game can be in.
#[derive(PartialEq)]
pub enum StateMachine {
    Play,
    Win,
    Loss,
    Menu(MenuData),
    Credits,
}

/// Both friendly and enemy bullets, because they share a bit in common with eachother.
pub struct Bullet {
    pub transform: Transform,
    pub invader: bool,
    delete: bool,
}

/// Defines a position and size.
/// Only use this for objects intended to be collided with. You don't need to use this on every single object/struct.
#[derive(Clone, Copy)]
pub struct Transform {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Transform {
    /// Use AABB collision to check if self is colliding with any other transform.
    /// You can also offset self, useful for the invaders.
    pub fn collided(&self, target: &Transform, offset_x: usize, offset_y: usize) -> bool {
        // Calculate the AABB collision
        let self_left = self.x + offset_x;
        let self_right = self.x + offset_x + self.width;
        let self_top = self.y + offset_y;
        let self_bottom = self.y + offset_y + self.height;

        let target_left = target.x;
        let target_right = target.x + target.width;
        let target_top = target.y;
        let target_bottom = target.y + target.height;

        // Check for collision in both the X and Y axes
        let x_collision = self_right >= target_left && self_left <= target_right;
        let y_collision = self_bottom >= target_top && self_top <= target_bottom;

        // If there's a collision in both axes, the two AABBs are colliding
        x_collision && y_collision
    }
}

impl Bullet {
    /// Marks the bullet for deletion, but doesn't actually delete it.
    pub fn delete(&mut self) {
        self.delete = true;
    }

    /// Creates a new bullet.
    pub fn new(x: usize, y: usize, invader: bool) -> Bullet {
        Bullet {
            transform: Transform {
                x,
                y,
                width: 1,
                height: 1,
            },
            invader,
            delete: false,
        }
    }
}

/// Invader.
pub struct Invader {
    pub transform: Transform,

    /// Should only be set to 10, 20, or 30. Not using an enum for conveniency.
    pub score: u8,
}

/// An enum defining which direction the horde of invaders should move.
#[derive(PartialEq)]
pub enum InvaderDirection {
    Left,
    Right,
}

/// The full group of invaders.
pub struct InvadersGroup {
    pub invaders: Vec<Vec<Invader>>,
    pub width: usize,
    pub direction: InvaderDirection,
    pub x: usize,
    pub y: usize,
}

/// An explosion effect.
pub struct Explosion {
    pub x: usize,
    pub y: usize,
    pub stage: usize,
    pub timer: u8,
}

#[derive(Clone, Copy)]
pub struct Wall {
    pub transform: Transform,
    pub health: u8,
}
/// Special Visual Effects.
/// This may be edited by the render thread when said effects are no longer needed.
pub struct Effects {
    pub explosions: Vec<Explosion>,
}
/// The game struct with all the information.
/// This struct is shared across the input, render, and game threads.
pub struct Game {
    pub ship: Transform,
    pub bullets: Vec<Bullet>,
    pub invaders_group: InvadersGroup,
    pub score: u64,
    pub lives: u8,
    pub state: StateMachine,
    pub effects: Effects,
    pub walls: [Wall; 4],
    pub invincible: bool,
    pub invincible_timer: usize,

    pub invader_move_timer: usize,

    pub width: usize,
    pub height: usize,
}

impl Game {
    pub fn init(width: usize, height: usize) -> Game {
        let mut invaders = Vec::new();

        for i in 0..5 {
            invaders.push(Vec::new());

            for x in (0..width - (14 * 4)).step_by(14) {
                if width - x <= 14 {
                    break;
                }

                let score;

                if i == 0 {
                    score = 30
                } else if i <= 2 {
                    score = 20
                } else {
                    score = 10
                }

                invaders[i].push(Invader {
                    transform: Transform {
                        x,
                        y: i * 10,
                        height: 8,
                        width: 10,
                    },
                    score,
                })
            }
        }

        let invaders_width = invaders[0].len() * 14;

        let mut walls = [Wall {
            transform: Transform {
                x: 0,
                y: height - 32,
                width: 26,
                height: 12,
            },
            health: 4,
        }; 4];

        for (i, wall) in walls.iter_mut().enumerate() {
            wall.transform.x = ((width / 4) * (i)) + (wall.transform.width / 4);
        }

        let game: Game = Game {
            state: StateMachine::Menu(DEFAULT_MENU),
            ship: Transform {
                x: 0,
                y: height - 16,
                width: 9,
                height: 10,
            },
            bullets: Vec::new(),
            effects: Effects {
                explosions: Vec::new(),
            },
            walls,
            invincible: false,
            invincible_timer: TICKS_TO_INVINCIBILITY,
            invaders_group: InvadersGroup {
                invaders,
                x: INVADER_PADDING,
                y: 12,
                width: invaders_width,
                direction: InvaderDirection::Right,
            },
            lives: 3,
            invader_move_timer: TICKS_TO_MOVE_INVADERS,
            score: 0,
            width,
            height,
        };

        game
    }
    pub fn tick(&mut self) {
        if self.state != StateMachine::Play {
            return;
        }

        if self.lives == 0 {
            self.state = StateMachine::Loss;

            return;
        } else if self.invaders_group.invaders.is_empty() {
            self.state = StateMachine::Win;

            return;
        }

        for i in 0..self.bullets.len() {
            if i >= self.bullets.len() {
                continue;
            }

            if self.bullets[i].transform.y == 0 || self.bullets[i].transform.y >= self.height - 4 {
                self.bullets[i].delete();

                continue;
            }

            for wall in &mut self.walls {
                if !wall.transform.collided(&self.bullets[i].transform, 0, 0) || wall.health == 0 {
                    continue;
                }

                wall.health -= 1;

                self.bullets[i].delete();

                self.effects.explosions.push(Explosion {
                    x: self.bullets[i].transform.x,
                    y: self.bullets[i].transform.y,
                    stage: 3,
                    timer: 8,
                });
            }

            if self.bullets[i].invader {
                if self.ship.collided(&self.bullets[i].transform, 0, 0) && !self.invincible {
                    crossterm::execute!(io::stdout(), crossterm::style::Print("\x07")).unwrap();
                    self.lives -= 1;
                    self.bullets[i].delete();

                    self.invincible = true;

                    continue;
                }

                self.bullets[i].transform.y += 1;
            } else {
                self.bullets[i].transform.y -= 1;
            }
        }

        self.bullets.retain(|bullet| !bullet.delete);

        self.invaders_group.invaders.retain_mut(|row| {
            if row.is_empty() {
                return false;
            }

            row.retain(|invader| {
                for bullet in &mut self.bullets {
                    if bullet.invader {
                        continue;
                    }

                    if invader.transform.collided(
                        &bullet.transform,
                        self.invaders_group.x,
                        self.invaders_group.y,
                    ) {
                        bullet.delete();

                        self.effects.explosions.push(Explosion {
                            x: bullet.transform.x,
                            y: bullet.transform.y,
                            stage: 3,
                            timer: 8,
                        });

                        self.score += invader.score as u64;

                        return false;
                    }
                }

                return true;
            });

            return true;
        });

        if self.invader_move_timer > 0 {
            self.invader_move_timer -= 1;

            return;
        } else {
            self.invader_move_timer = TICKS_TO_MOVE_INVADERS;
        }

        if self.invincible {
            if self.invincible_timer > 0 {
                self.invincible_timer -= 1;
            } else {
                self.invincible_timer = TICKS_TO_INVINCIBILITY;
                self.invincible = false;
            }
        }

        let mut rand = rand::thread_rng();

        if rand.gen_range(0..5) == 0 {
            let i = rand.gen_range(0..self.invaders_group.invaders.len());
            let j = rand.gen_range(0..self.invaders_group.invaders[i].len());

            let shooter = &self.invaders_group.invaders[i][j];

            self.bullets.push(Bullet::new(
                shooter.transform.x + 6 + self.invaders_group.x,
                shooter.transform.y + 5 + self.invaders_group.y,
                true,
            ));
        }

        if self.invaders_group.direction == InvaderDirection::Right {
            self.invaders_group.x += 1;

            if self.invaders_group.x + self.invaders_group.width + INVADER_PADDING > self.width {
                self.invaders_group.y += 4;
                self.invaders_group.direction = InvaderDirection::Left
            }
        } else {
            self.invaders_group.x -= 1;

            if self.invaders_group.x - INVADER_PADDING <= 2 {
                self.invaders_group.y += 4;
                self.invaders_group.direction = InvaderDirection::Right
            }
        }
    }
}
