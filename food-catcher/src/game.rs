use rand::Rng;

use crate::display;

#[derive(PartialEq, Eq)]
pub enum TickStatus {
    Ok,
    Exit,
    NoTick
}

pub struct Food {
    pub pos_y: usize,
    pub pos_x: usize,
}

impl Food {
    pub fn new(game: &Game) -> Food {
        let spawn_x = rand::thread_rng().gen_range(0..game.width);
        Food {
            pos_x: spawn_x,
            pos_y: 0,
        }
    }
}

pub struct Game {
    pub player_x: usize,
    pub score: u32,
    pub lives: u8,
    pub foods: Vec<Food>,

    pub width: usize,
    pub height: usize,

    // The amount of ticks that need to go by for a food to be spawned.
    pub ticks_for_food: u8,

    tick: u8,
    food_timer: u8,
}

impl Game {
    pub fn new(screen: &display::Screen) -> Game {
        Game {
            player_x: 10,
            tick: 0,
            food_timer: 0,
            ticks_for_food: 10,
            width: screen.width,
            height: screen.height,
            score: 0,
            lives: 3,
            foods: vec![],
        }
    }

    pub fn do_tick(&mut self) -> TickStatus {
        {
            if self.food_timer == self.ticks_for_food {
                self.food_timer = 0;

                self.spawn_food()
            }

            self.food_timer += 1;
        }

        self.foods.retain(|food| {
            let grounded = food.pos_y >= self.height - 3;
            let touching_player = food.pos_x >= self.player_x
                && food.pos_x <= self.player_x + 5
                && food.pos_y >= self.height - 7;

            if touching_player {
                self.score += 1;
            } else if grounded {
                self.lives -= 1;
            }

            !grounded && !touching_player
        });

        for food in &mut self.foods {
            food.pos_y += 1;
        }

        if self.lives <= 0 {
            return TickStatus::Exit
        }

        return TickStatus::Ok
    }

    pub fn spawn_food(&mut self) {
        self.foods.push(Food::new(&self))
    }

    // Ticks every 20 frames, so 200 ms per tick.
    pub fn tick(&mut self) -> TickStatus {
        if self.tick == 20 {
            self.tick = 0;

            return self.do_tick()
        }

        self.tick += 1;

        return TickStatus::NoTick
    }
}
