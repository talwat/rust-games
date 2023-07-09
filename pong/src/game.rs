use rand::Rng;

use crate::{screen, PADDLE_PADDING};

// Simplifies an angle to be from 1 to 360 degrees.
// Favors 360 degrees instead of 0 degrees
pub fn angle(raw: f32) -> f32 {
    let mut result = raw;

    while result >= 360.0 {
        result -= 360.0
    }

    while result <= 0.0 {
        result += 360.0
    }

    return result;
}

pub struct Game {
    pub paddle_1_y: usize,
    pub paddle_2_y: usize,
    pub score_1: u32,
    pub score_2: u32,
    pub server: bool,
    pub ball_x: f32,
    pub ball_y: f32,
    pub ball_dir: f32,

    pub width: usize,
    width_f32: f32,
    pub height: usize,
    height_f32: f32,
}

impl Game {
    pub fn new(screen: &screen::Screen) -> Game {
        Game {
            paddle_1_y: 0,
            paddle_2_y: 0,
            ball_dir: 0.0,
            ball_x: 0.0,
            ball_y: 0.0,
            server: false,
            width: screen.width,
            width_f32: screen.width as f32,
            height: screen.height - 1,
            height_f32: (screen.height - 1) as f32,

            score_1: 0,
            score_2: 0,
        }
    }

    pub fn ball_initial_pos(&mut self) {
        self.ball_x = self.width as f32 / 2.0;
        self.ball_y = self.height as f32 / 2.0;

        if self.server {
            self.ball_dir = angle(180.0);
        } else {
            self.ball_dir = angle(180.0);
        }

        self.server = !self.server;
    }

    pub fn tick(&mut self) {
        if self.ball_y.round() >= self.height_f32 {
            self.ball_dir = angle(self.ball_dir - 180.0)
        }

        if self.ball_y.round() <= 0.0 {
            self.ball_dir = angle(self.ball_dir - 180.0)
        }

        if self.ball_x.round() >= self.width_f32 - 1.0 {
            self.score_1 += 1;
            self.ball_initial_pos();
        }

        if self.ball_x.round() <= 0.0 {
            self.score_2 += 1;
            self.ball_initial_pos();
        }

        const PADDLE_PADDING_F32: f32 = PADDLE_PADDING as f32;

        // Perform some neat trigonometry to move the ball in a certain direction that I don't understand :)
        self.ball_y = self.ball_y + self.ball_dir.to_radians().sin();

        let cos = self.ball_dir.to_radians().cos();

        if self.ball_dir >= 180.0 {
            self.ball_x -= cos;
        } else {
            self.ball_x += cos;
        }

        let offset;
        let ball_rounded = self.ball_y.ceil() as i16;

        // Collided with paddle 1
        let collided_1 = self.ball_x.floor() == PADDLE_PADDING_F32 + 1.0;

        // Collided with paddle 2
        let collided_2 = self.ball_x.floor() == self.width_f32 - PADDLE_PADDING_F32 - 3.0;

        if collided_1 {
            offset = self.paddle_1_y as i16 - ball_rounded;
        } else if collided_2 {
            offset = self.paddle_2_y as i16 - ball_rounded;
        } else {
            return;
        }

        let mut ball_angle;

        match offset {
            -2 => ball_angle = 45.0,
            -1 => ball_angle = 22.5,
            0 => ball_angle = 180.0,
            1 => ball_angle = 180.0 + 22.5,
            2 => ball_angle = 180.0 + 45.0,
            _ => return,
        }

        if collided_2 {
            ball_angle = 180.0 - ball_angle;
        }

        let mut rng = rand::thread_rng();

        let rand_offset: f32 = rng.gen_range(-10..10) as f32;

        ball_angle += rand_offset;

        self.ball_dir = angle(ball_angle);
    }

    pub fn move_paddle_1(&mut self, amount: i16) {
        let new = self.paddle_1_y as i16 + amount;

        if new <= 0 && new >= self.height as i16 {
            return;
        }

        self.paddle_1_y = new as usize;
    }

    pub fn move_paddle_2(&mut self, amount: i16) {
        let new = self.paddle_2_y as i16 + amount;

        if new <= 0 && new >= self.height as i16 {
            return;
        }

        self.paddle_2_y = new as usize;
    }
}
