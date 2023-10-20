//! Deals With the... psf2 format??? Jeez, this is really random.
//! The psfu format is what's used in the linux tty.
//! You can find the built in psf2 fonts in /usr/share/kbd/consolefonts.
//! 
//! This doesn't support the original psf, and currently doesn't support glyphs that aren't 8px wide.

use std::fmt;

#[derive(Debug, Clone)]
pub enum FontError {
    IncorrectHeader
}

impl fmt::Display for FontError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FontError::IncorrectHeader => write!(fmt, "header magic does not match, is this a psf2 font?"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub magic: [u8; 4],
    pub version: u32,
    pub size: u32,
    pub flags: u32,
    pub length: u32,
    pub glyph_size: u32,
    pub glyph_height: u32,
    pub glyph_width: u32,
}

#[derive(Debug)]
pub struct Font {
    pub header: Header,
    char_data: Box<[u8]>,
}

impl Font {
    pub fn get_char(&self, char: char) -> Vec<Vec<u8>> {
        let char = char as u32;

        let mut result: Vec<Vec<u8>> = Vec::with_capacity(self.header.glyph_size as usize);

        let from = self.header.glyph_size * (char);
        let to = self.header.glyph_size * (char+1);

        for (i, byte) in self.char_data[from as usize..to as usize].iter().enumerate() {
            if byte == &0 {
                result.push(Vec::with_capacity(0));

                continue;
            }
            
            result.push(Vec::with_capacity(8));

            for j in 0..8 {
                let bit = (byte >> 7 - j) & 1;

                result[i].push(bit);
            }
        }

        result
    }

    pub fn load(raw: &[u8]) -> Result<Font, FontError> {
        let font = Font {
            header: Header {
                magic: [raw[0x0], raw[0x1], raw[0x2], raw[0x3]],
                version: as_u32_le(&raw[0x4..0x8]),
                size: as_u32_le(&raw[0x8..0xc]),
                flags: as_u32_le(&raw[0xc..0x10]),
                length: as_u32_le(&raw[0x10..0x14]),
                glyph_size: as_u32_le(&raw[0x14..0x18]),
                glyph_height: as_u32_le(&raw[0x18..0x1c]),
                glyph_width: as_u32_le(&raw[0x1c..0x20]),
            },
            char_data: (&raw[32..]).into(),
        };

        if font.header.magic != [0x72, 0xb5, 0x4a, 0x86] {
            return Result::Err(FontError::IncorrectHeader);
        }

        Result::Ok(font)
    }
}

fn as_u32_le(array: &[u8]) -> u32 {
    ((array[0] as u32) << 0)
        + ((array[1] as u32) << 8)
        + ((array[2] as u32) << 16)
        + ((array[3] as u32) << 24)
}
