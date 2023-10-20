const TICKS_TO_MOVE_INVADERS: usize = 12;

pub struct Bullet {
    pub x: usize,
    pub y: usize,
    delete: bool,
}

impl Bullet {
    pub fn delete(&mut self) {
        self.delete = true;
    }

    pub fn new(x: usize, y: usize) -> Bullet {
        Bullet {
            x,
            y,
            delete: false,
        }
    }
}

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq)]
pub enum InvaderDirection {
    Left,
    Right,
}

pub struct InvadersGroup {
    pub invaders: Vec<Vec<Invader>>,
    pub width: usize,
    pub direction: InvaderDirection,
    pub x: usize,
    pub y: usize,
}

pub struct Game {
    pub x: usize,
    pub bullets: Vec<Bullet>,
    pub invaders_group: InvadersGroup,
    pub score: u64,

    pub invader_move_timer: usize,

    pub width: usize,
    pub height: usize,
}

impl Game {
    pub fn init(width: usize, height: usize) -> Game {
        let mut invaders = Vec::new();

        for (i, y) in (0..height / 3).step_by(10).enumerate() {
            invaders.push(Vec::new());

            for x in (0..width - (14 * 4)).step_by(14) {
                if width - x <= 14 {
                    break;
                }
                invaders[i].push(Invader { x, y })
            }
        }

        let invaders_width = invaders[0].len() * 14;

        let game = Game {
            x: 0,
            bullets: Vec::new(),
            invaders_group: InvadersGroup {
                invaders,
                x: 0,
                y: 0,
                width: invaders_width,
                direction: InvaderDirection::Right,
            },
            invader_move_timer: TICKS_TO_MOVE_INVADERS,
            score: 0,
            width,
            height,
        };

        game
    }
    pub fn tick(&mut self) {
        for i in 0..self.bullets.len() {
            if i >= self.bullets.len() {
                continue;
            }

            if self.bullets[i].y == 0 || self.bullets[i].delete {
                self.bullets.remove(i);
            } else {
                self.bullets[i].y -= 1;
            }
        }

        let invader_group_x = self.invaders_group.x;

        for invader_row in &mut self.invaders_group.invaders {
            invader_row.retain(|invader| {
                let mut collided = false;

                for bullet in &mut self.bullets {
                    if !collided {
                        collided = (bullet.x >= invader.x + invader_group_x
                            && bullet.x <= invader.x + invader_group_x + 12)
                            && (bullet.y >= invader.y && bullet.y <= invader.y + 8);

                        if collided {
                            bullet.delete = true;

                            self.score += 1;
                        }
                        break;
                    }
                }

                !collided
            });
        }

        if self.invader_move_timer > 0 {
            self.invader_move_timer -= 1;

            return;
        } else {
            self.invader_move_timer = TICKS_TO_MOVE_INVADERS;
        }

        if self.invaders_group.direction == InvaderDirection::Right {
            self.invaders_group.x += 1;

            if self.invaders_group.x + self.invaders_group.width >= self.width {
                self.invaders_group.y += 4;
                self.invaders_group.direction = InvaderDirection::Left
            }
        } else {
            self.invaders_group.x -= 1;

            if self.invaders_group.x <= 2 {
                self.invaders_group.y += 4;
                self.invaders_group.direction = InvaderDirection::Right
            }
        }
    }
}
