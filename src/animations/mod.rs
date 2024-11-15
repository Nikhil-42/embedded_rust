#[allow(deprecated)]
use core::hash::SipHasher;
use core::hash::{Hash, Hasher};

use smart_leds::{
    hsv::{hsv2rgb, Hsv},
    RGB8,
};

const NUM_PIXELS: usize = 64;
const PIXEL_WIDTH: usize = 3;

pub fn rotate_90(data: [RGB8; 64]) -> [RGB8; 64] {
    let mut temp = [RGB8::default(); 64];
    for i in 0..8 {
        for j in 0..8 {
            let index = i * 8 + j;
            let new_index = j * 8 + (7 - i);
            temp[new_index] = data[index];
        }
    }
    temp
}

pub fn rotate_180(data: [RGB8; 64]) -> [RGB8; 64] {
    let mut temp = [RGB8::default(); 64];
    for i in 0..8 {
        for j in 0..8 {
            let index = i * 8 + j;
            let new_index = (7 - i) * 8 + (7 - j);
            temp[new_index] = data[index];
        }
    }
    temp
}

pub fn rotate_270(data: [RGB8; 64]) -> [RGB8; 64] {
    let mut temp = [RGB8::default(); 64];
    for i in 0..8 {
        for j in 0..8 {
            let index = i * 8 + j;
            let new_index = (7 - j) * 8 + i;
            temp[new_index] = data[index];
        }
    }
    temp
}

pub trait Animation {
    fn next(&mut self);
    fn to_list(&self) -> [RGB8; NUM_PIXELS];
}

#[derive(Default)]
pub struct Rainbow {
    hue: u8,
}

impl Animation for Rainbow {
    fn next(&mut self) {
        self.hue = self.hue.wrapping_add(1);
    }

    fn to_list(&self) -> [RGB8; NUM_PIXELS] {
        let mut data = [RGB8::default(); NUM_PIXELS];
        for (i, pixel) in data.iter_mut().enumerate() {
            *pixel = hsv2rgb(Hsv {
                hue: self.hue.wrapping_add(i as u8 * 4),
                sat: 255,
                val: 32,
            });
        }
        data
    }
}
pub const BADAPPLE_FRAMES: &[u8; 1262400] = include_bytes!("badapple.raw");
// pub const PULSE_FRAMES: &[u8; 18432] = include_bytes!("pulse.raw");
pub const RICK_ROLL: &[u8; 36480] = include_bytes!("rick_roll.raw");

pub struct FromRaw {
    data: &'static [u8],
    frame: usize,
    num_frames: usize,
}

impl FromRaw {
    pub fn new(data: &'static [u8]) -> Self {
        if data.len() % (NUM_PIXELS * PIXEL_WIDTH) != 0 {
            panic!("Data length must be a multiple of 64 (pixels), and 3 (RGB)!");
        }
        Self {
            data,
            frame: 0,
            num_frames: data.len() / NUM_PIXELS / PIXEL_WIDTH,
        }
    }
}

impl Animation for FromRaw {
    fn next(&mut self) {
        self.frame = (self.frame + 1) % self.num_frames;
    }

    fn to_list(&self) -> [RGB8; NUM_PIXELS] {
        let mut data = [RGB8::default(); NUM_PIXELS];
        for (i, pixel) in data.iter_mut().enumerate() {
            let offset = self.frame * NUM_PIXELS * PIXEL_WIDTH + i * PIXEL_WIDTH;
            *pixel = RGB8 {
                r: self.data[offset + 0],
                g: self.data[offset + 1],
                b: self.data[offset + 2],
            };
        }
        data
    }
}

#[derive(Default)]
pub struct Squares {
    frame: usize,
    increment_counter: usize,
    direction: bool,
}

impl Animation for Squares {
    fn next(&mut self) {
        self.increment_counter = (self.increment_counter + 1) % 24;
        if self.increment_counter == 0 {
            if self.direction {
                if self.frame <= 0 {
                    self.direction = false;
                    self.frame += 1;
                } else {
                    self.frame -= 1;
                }
            } else {
                if self.frame >= 3 {
                    self.direction = true;
                    self.frame -= 1;
                } else {
                    self.frame += 1;
                }
            }
        }
    }

    fn to_list(&self) -> [RGB8; NUM_PIXELS] {
        let mut data = [RGB8::default(); NUM_PIXELS];
        for (i, pixel) in data.iter_mut().enumerate() {
            let x = i % 8;
            let y = i / 8;
            if ((x == self.frame || x == 7 - self.frame)
                && (y >= self.frame && y <= 7 - self.frame))
                || ((y == self.frame || y == 7 - self.frame)
                    && (x >= self.frame && x <= 7 - self.frame))
            {
                *pixel = (255, 255, 255).into();
            }
        }
        data
    }
}

pub struct Life {
    data: [[bool; 8]; 8],
    increment_counter: usize,
}

impl Default for Life {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Life {
    pub fn new(key1: u64, key2: u64) -> Self {
        let mut data = [[false; 8]; 8];
        #[allow(deprecated)]
        let mut hasher = SipHasher::new_with_keys(key1, key2);

        for (i, row) in data.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                (j + 8 * i).hash(&mut hasher);
                *cell = hasher.finish() % 2 == 0;
            }
        }
        Self {
            data,
            increment_counter: 0,
        }
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dx in 0..3 {
            for dy in 0..3 {
                if dx == 1 && dy == 1 {
                    continue;
                }
                let nx = (x + dx + 7) % 8;
                let ny = (y + dy + 7) % 8;
                if self.data[nx][ny] {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Animation for Life {
    fn next(&mut self) {
        self.increment_counter = (self.increment_counter + 1) % 24;
        if !(self.increment_counter == 0 || self.increment_counter == 12) {
            return;
        }
        let mut new_data = [[false; 8]; 8];
        for i in 0..8 {
            for j in 0..8 {
                let neighbors = self.count_neighbors(i, j);
                new_data[i][j] = (self.data[i][j] && neighbors == 2) || neighbors == 3;
            }
        }
        self.data = new_data;
    }

    fn to_list(&self) -> [RGB8; NUM_PIXELS] {
        let mut data = [RGB8::default(); NUM_PIXELS];
        for (i, pixel) in data.iter_mut().enumerate() {
            let x = i % 8;
            let y = i / 8;
            if self.data[x][y] {
                *pixel = (255, 255, 255).into();
            }
        }
        data
    }
}
