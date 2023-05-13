use std::io::{self, Write};

use console::Term;

use crate::tile::{BgColor, FgColor, Tile};

pub struct Screen {
    pub height: usize,
    pub width: usize,

    // The screen which has sprites and other foreground elements drawn in real time.
    data: Vec<Vec<[u8; 11]>>,

    // The screen initially. This is used for clearing the screen.
    // So whenever the screen needs to be wiped, this can be used.
    // You can wipe the screen with the reset() method.
    initial: Vec<Vec<[u8; 11]>>,

    stdout: std::io::Stdout,
}

impl Screen {
    pub fn new(term: &Term) -> Screen {
        // Get the terminal dimentions.
        let (height, width) = term.size();

        let height: usize = (height - 1).into();
        let width: usize = width.into();

        let initial = Vec::new();
        let data: Vec<Vec<[u8; 11]>> = Vec::new();

        let mut screen = Screen {
            data,
            height,
            width,
            initial,
            stdout: io::stdout(),
        };

        screen.initial_draw();

        screen.initial = screen.data.clone();

        return screen;
    }

    // Draws the initial background elements like the sky and ground.
    pub fn initial_draw(&mut self) {
        self.bg_color(Tile::new(FgColor::Black, BgColor::Blue, b'-'));
        self.line(
            self.height - 2,
            Tile::new(FgColor::Black, BgColor::Yellow, b'='),
        );
        self.line(
            self.height - 1,
            Tile::new(FgColor::Black, BgColor::Yellow, b'='),
        );
    }

    pub fn render(&self, status: &str) {
        // Rushing out the output is actually faster than collecting it and then outputting it, and it leads to less flicker.

        // We can lock the standard output which allows us to print to it as much as we
        // want. It's automatically unlocked at the end of this function.
        // TODO: Consider locking it for the entire duration of the program.
        let mut lock = self.stdout.lock();

        // Clear screen & return to the home position.
        // It's the best way to do it, and it has the bonus of keeping your terminal
        // history clean.
        lock.write_all(b"\x1b[J\x1b[H").unwrap();

        // Iterate over every single tile and print it out.
        for line in &self.data {
            for tile in line {
                lock.write_all(tile).unwrap();
            }
        }

        // Status bar. It just makes a newline and resets the colors before printing.
        lock.write_all((["\n\x1b[0m",status].join("")).as_bytes())
            .unwrap();

        // Flushes everything and cleans everything out.
        // It improves a couple of things, like making sure the status bar is printed out properly.
        lock.flush().unwrap();
    }

    // Takes the initial state of the screen and loads it into the current state.
    // It basically just wipes any modifications made to the screen.
    pub fn reset(&mut self) {
        self.data = self.initial.clone();
    }

    // Sets actual bytes instead of using the abstracted tile.
    fn set_bytes(&mut self, x: usize, y: usize, tile: [u8; 11]) {
        self.data[y][x] = tile;
    }

    // More cleanly sets a tile.
    pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.set_bytes(x, y, tile.calc());
    }

    // Draws a horizontal line across the screen.
    pub fn line(&mut self, y: usize, tile: Tile) {
        self.data[y] = std::vec::from_elem(tile.calc(), self.width);
    }

    // Draws a rectangle
    pub fn rectangle(
        &mut self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
        tile: Tile,
    ) {
        let calculated_tile = tile.calc();

        for y in from_y..to_y {
            for x in from_x..to_x {
                self.set_bytes(x, y, calculated_tile)
            }
        }
    }

    // Wipes the screen and replaces it with one solid tile.
    pub fn bg_color(&mut self, tile: Tile) {
        self.data = std::vec::from_elem(std::vec::from_elem(tile.calc(), self.width), self.height)
    }
}
