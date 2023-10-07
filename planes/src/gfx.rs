use std::{
    io::{self, Write},
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
    time::{Duration, Instant}, f32::consts::PI, path, iter,
};

use image::GenericImageView;

#[derive(Copy, Clone, Debug)]
pub struct RGB(pub u8, pub u8, pub u8);

type Pixel = RGB;
type LoadedImage = Vec<Vec<Option<Pixel>>>;

pub enum ChannelMessage {
    Stop,
}

pub struct Screen {
    pub height: usize,
    pub width: usize,

    // The screen which has sprites and other foreground elements drawn in real time.
    data: Vec<Vec<Pixel>>,

    // The screen initially. This is used for clearing the screen.
    // So whenever the screen needs to be wiped, this can be used.
    // You can wipe the screen with the reset() method.
    initial: Vec<Vec<Pixel>>,

    // The output to write to
    out: std::io::Stdout,
}

impl Screen {
    pub fn new(mut initial_draw: impl FnMut(&mut Screen)) -> Screen {
        // Get the terminal dimentions.
        let (width, height) = crossterm::terminal::size().unwrap();

        let height: usize = (height * 2).into();
        let width: usize = width.into();

        let initial = Vec::with_capacity(height);
        let data = Vec::with_capacity(height);

        let mut screen = Screen {
            data,
            height,
            width,
            initial,
            out: io::stdout(),
        };

        initial_draw(&mut screen);

        screen.initial = screen.data.clone();

        return screen;
    }

    pub fn update(
        mut self,
        receiver: Receiver<ChannelMessage>,
        mut action: impl FnMut(&mut Screen) + 'static + std::marker::Send,
    ) -> JoinHandle<Screen> {
        thread::spawn(move || loop {
            if let Ok(msg) = receiver.try_recv() {
                match msg {
                    ChannelMessage::Stop => {
                        return self;
                    }
                }
            }

            let now = Instant::now();

            self.render();
            self.reset();
            action(&mut self);

            const TARGET_DELTA: Duration = Duration::from_millis(16);

            if now.elapsed() < TARGET_DELTA {
                thread::sleep(TARGET_DELTA - now.elapsed())
            }
        })
    }

    pub fn render(&self) {
        // Rushing out the output is actually faster than collecting it and then outputting it, and it leads to less flicker.

        // We can lock the standard output which allows us to print to it as much as we
        // want. It's automatically unlocked at the end of this function.
        // TODO: Consider locking it for the entire duration of the program.
        let mut lock = self.out.lock();

        // Clear screen & return to the home position.
        // It's the best way to do it, and it has the bonus of keeping your terminal
        // history clean.
        lock.write_all(b"\x1b[J\x1b[H").unwrap();

        // Iterate over every single tile and print it out.
        for y in (0..self.height - 1).step_by(2) {
            for x in 0..self.width {
                let pixel = self.data[y][x];
                let next = self.data[y + 1][x];
                lock.write_fmt(format_args!(
                    "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▄",
                    next.0, next.1, next.2, pixel.0, pixel.1, pixel.2
                ))
                .unwrap();
            }
        }

        lock.flush().unwrap();
    }

    // Takes the initial state of the screen and loads it into the current state.
    // It basically just wipes any modifications made to the screen.
    pub fn reset(&mut self) {
        self.data = self.initial.clone();
    }

    // More cleanly sets a tile.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: RGB) {
        self.data[y][x] = color;
    }

    // Draws a circle.
    pub fn circle(&mut self, x: usize, y: usize, radius: usize, color: RGB) {
        for i in 0..360 {
            let angle = i as f32;
            let x1 = radius as f32 * (angle * PI / 180.0).cos();
            let y1 = radius as f32 * (angle * PI / 180.0).sin();
            self.set_pixel((x as f32 + x1).round() as usize, (y as f32 + y1).round() as usize, color);
        }
    }

    pub fn load_image(path: &str) -> LoadedImage {
        let img = image::open(path).unwrap();
        let (width, height) = img.dimensions();
        let mut result: LoadedImage = vec!(vec!(None; width as usize); height as usize);
 
        for (x, y, pixel) in img.pixels() {
            if pixel[3] != 0 {
                result[y as usize][x as usize] = Some(RGB(pixel[0], pixel[1], pixel[2]));
            }
        }

        result
    }

    pub fn image(&mut self, x: usize, y: usize, image: &LoadedImage, mirror_horizontal: bool, mirror_vertical: bool) {
        let mut final_image = image.clone();

        if mirror_vertical {
            final_image.reverse();
        }

        for (y1, row) in final_image.iter().enumerate() {
            let mut final_row = row.clone();

            if mirror_horizontal {
                final_row.reverse();
            }

            for (x1, pixel) in final_row.iter().enumerate() {
                if let Some(pixel) = pixel {
                    self.set_pixel(x+x1, y+y1, *pixel)
                }
            }
        }
    }

    // Draws a rectangle.
    pub fn rectangle(
        &mut self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
        color: RGB,
    ) {
        for y in from_y..to_y {
            for x in from_x..to_x {
                self.set_pixel(x, y, color)
            }
        }
    }

    // Wipes the screen and replaces it with one solid color.
    pub fn bg(&mut self, color: RGB) {
        self.data = vec!(vec!(color; self.width); self.height);
    }
}
