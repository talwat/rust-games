pub enum FgColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Default,
}

pub enum BgColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Default,
}

impl FgColor {
    pub fn get_code(&self) -> [u8; 2] {
        match self {
            FgColor::Black => *b"30",
            FgColor::Red => *b"31",
            FgColor::Green => *b"32",
            FgColor::Yellow => *b"33",
            FgColor::Blue => *b"34",
            FgColor::Default => *b"39",
        }
    }
}

impl BgColor {
    pub fn get_code(&self) -> [u8; 2] {
        match self {
            BgColor::Black => *b"40",
            BgColor::Red => *b"41",
            BgColor::Green => *b"42",
            BgColor::Yellow => *b"43",
            BgColor::Blue => *b"44",
            BgColor::Default => *b"49",
        }
    }
}

pub struct Tile {
    pub fg_color: FgColor,
    pub bg_color: BgColor,
    pub tile: u8,
}

impl Tile {
    pub fn new(fg_color: FgColor, bg_color: BgColor, tile: u8) -> Tile {
        Tile {
            fg_color,
            bg_color,
            tile,
        }
    }

    pub fn calc(&self) -> [u8; 11] {
        let fg_code = self.fg_color.get_code();
        let bg_code = self.bg_color.get_code();

        [
            b'\x1b', b'[', b'0', b';', fg_code[0], fg_code[1], b';', bg_code[0], bg_code[1], b'm',
            self.tile,
        ]
    }
}
