use rand::Rng;

use crate::{exit, screen, PIPE_GAP, PIPE_WIDTH, PLAYER_SPAWN_X};

pub struct Pipes {
    pub pos_x: usize,
    pub offset_y: usize,
    pub scored: bool,
}

impl Pipes {
    pub fn new(game: &Game) -> Pipes {
        let offset: usize = rand::thread_rng().gen_range(5..game.height - 5);

        Pipes {
            pos_x: game.width - 3,
            offset_y: offset,
            scored: false,
        }
    }
}

pub struct Game {
    pub player_y: usize,
    pub score: u32,
    pub pipes: Vec<Pipes>,

    pub width: usize,
    pub height: usize,

    flap: u8,
    spawn_pipe_timer: u8,
}

impl Game {
    pub fn new(screen: &screen::Screen) -> Game {
        Game {
            player_y: 10,
            flap: 0,
            spawn_pipe_timer: 0,
            width: screen.width,
            height: screen.height,
            score: 0,
            pipes: vec![],
        }
    }

    pub fn flap(&mut self) {
        self.flap = 2;
    }

    pub fn tick(&mut self) {
        for pipe in &mut self.pipes {
            pipe.pos_x -= 1;
        }

        if self.flap > 0 {
            self.player_y -= 1;
            self.flap -= 1;
        } else {
            self.player_y += 1;
        }

        self.pipes.retain(|pipe| pipe.pos_x > 0);

        if self.spawn_pipe_timer <= 0 {
            self.spawn_pipe_timer = 20;

            self.pipes.push(Pipes::new(&self))
        } else {
            self.spawn_pipe_timer -= 1;
        }

        let pipe = &mut self.pipes[0];

        if ((PLAYER_SPAWN_X >= pipe.pos_x && PLAYER_SPAWN_X <= pipe.pos_x + PIPE_WIDTH)
            && (self.player_y >= pipe.offset_y + PIPE_GAP
                || self.player_y <= pipe.offset_y - PIPE_GAP - 1))
            || (self.player_y == 0 || self.player_y == self.height)
        {
            exit(&format!("Game Over! You had a score of: {}", self.score))
        }

        if PLAYER_SPAWN_X >= pipe.pos_x + 1 && !pipe.scored {
            self.score += 1;

            pipe.scored = true;
        }
    }
}
