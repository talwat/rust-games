use std::io::{self, Write};

use crate::game;
use console::Term;

pub struct Screen {
    data: Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
}

impl Screen {
    pub fn new(term: &Term) -> Screen {
        let (mut height, width) = term.size();
        height -= 1;
    
        let height: usize = height.into();
        let width: usize = width.into();
    
        let mut data: Vec<Vec<char>> = std::vec::from_elem(
            std::vec::from_elem(
                ' ', width
            ),
            height
        );

        data[height-2] = std::vec::from_elem(
            '-', width
        );
        
        Screen { data, height, width }
    }

    pub fn render_stickman(&mut self, game: &game::Game, clear: bool) {
        let player_y = self.height - 6;

        const STICKMAN: [[char; 5]; 4] = [
            ['(', 'u', 'w', 'u', ')'],
            [' ', '/', '|', '\\', ' '],
            [' ', ' ', '|', ' ', ' '],
            [' ', '/', ' ', '\\', ' '],
        ];

        for (i, row) in STICKMAN.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if clear {
                    self.data[player_y+i][game.player_x+j] = ' '
                } else {
                    self.data[player_y+i][game.player_x+j] = *char;
                }
            }
        }
    }

    pub fn render_food(&mut self, game: &game::Game, clear: bool) {
        for food in &game.foods {
            if clear {
                self.data[food.pos_y][food.pos_x] = ' '
            } else {
                self.data[food.pos_y][food.pos_x] = '@'
            }
        }
    }

    pub fn render(&mut self, game: &game::Game) {
        let mut output: String = String::from("\x1b[H\r\x1b[2J\r");
        
        self.render_stickman(game, false);
        self.render_food(game, false);
        
        for line in &self.data {
            output.push_str(&line.iter().collect::<String>());
        };
        
        self.render_stickman(game, true);
        self.render_food(game, true);
        
        output.push_str(&format!("Score: {}  Lives: {}", game.score, game.lives));
        
        io::stdout().write_all(output.as_bytes()).unwrap();
    }
}