pub const CHAR_WIDTH: usize  = 3;
pub const CHAR_HEIGHT: usize = 5;
const SPACING: usize = 1;

#[rustfmt::skip]
const GLYPHS: [[u8; 15]; 40] = [
    // 0-9
    [1,1,1, 1,0,1, 1,0,1, 1,0,1, 1,1,1],
    [0,1,0, 1,1,0, 0,1,0, 0,1,0, 1,1,1],
    [1,1,1, 0,0,1, 1,1,1, 1,0,0, 1,1,1],
    [1,1,1, 0,0,1, 1,1,1, 0,0,1, 1,1,1],
    [1,0,1, 1,0,1, 1,1,1, 0,0,1, 0,0,1],
    [1,1,1, 1,0,0, 1,1,1, 0,0,1, 1,1,1],
    [1,1,1, 1,0,0, 1,1,1, 1,0,1, 1,1,1],
    [1,1,1, 0,0,1, 0,1,0, 0,1,0, 0,1,0],
    [1,1,1, 1,0,1, 1,1,1, 1,0,1, 1,1,1],
    [1,1,1, 1,0,1, 1,1,1, 0,0,1, 1,1,1],
    // A-Z
    [0,1,0, 1,0,1, 1,1,1, 1,0,1, 1,0,1], // A
    [1,1,0, 1,0,1, 1,1,0, 1,0,1, 1,1,0], // B
    [0,1,1, 1,0,0, 1,0,0, 1,0,0, 0,1,1], // C
    [1,1,0, 1,0,1, 1,0,1, 1,0,1, 1,1,0], // D
    [1,1,1, 1,0,0, 1,1,0, 1,0,0, 1,1,1], // E
    [1,1,1, 1,0,0, 1,1,0, 1,0,0, 1,0,0], // F
    [0,1,1, 1,0,0, 1,0,1, 1,0,1, 0,1,1], // G
    [1,0,1, 1,0,1, 1,1,1, 1,0,1, 1,0,1], // H
    [1,1,1, 0,1,0, 0,1,0, 0,1,0, 1,1,1], // I
    [0,1,1, 0,0,1, 0,0,1, 1,0,1, 0,1,0], // J
    [1,0,1, 1,0,1, 1,1,0, 1,0,1, 1,0,1], // K
    [1,0,0, 1,0,0, 1,0,0, 1,0,0, 1,1,1], // L
    [1,0,1, 1,1,1, 1,0,1, 1,0,1, 1,0,1], // M
    [1,0,1, 1,1,1, 1,1,1, 1,0,1, 1,0,1], // N
    [0,1,0, 1,0,1, 1,0,1, 1,0,1, 0,1,0], // O
    [1,1,0, 1,0,1, 1,1,0, 1,0,0, 1,0,0], // P
    [0,1,0, 1,0,1, 1,0,1, 1,1,1, 0,1,1], // Q
    [1,1,0, 1,0,1, 1,1,0, 1,0,1, 1,0,1], // R
    [0,1,1, 1,0,0, 0,1,0, 0,0,1, 1,1,0], // S
    [1,1,1, 0,1,0, 0,1,0, 0,1,0, 0,1,0], // T
    [1,0,1, 1,0,1, 1,0,1, 1,0,1, 1,1,1], // U
    [1,0,1, 1,0,1, 1,0,1, 0,1,0, 0,1,0], // V
    [1,0,1, 1,0,1, 1,0,1, 1,1,1, 1,0,1], // W
    [1,0,1, 1,0,1, 0,1,0, 1,0,1, 1,0,1], // X
    [1,0,1, 1,0,1, 0,1,0, 0,1,0, 0,1,0], // Y
    [1,1,1, 0,0,1, 0,1,0, 1,0,0, 1,1,1], // Z
    // Special_
    [0,0,0, 0,0,0, 1,1,1, 0,0,0, 0,0,0], // -
    [0,0,0, 0,1,0, 1,1,1, 0,1,0, 0,0,0], // +
    [0,0,1, 0,0,1, 0,1,0, 1,0,0, 1,0,0], // /
    [0,0,0, 0,1,0, 0,0,0, 0,0,0, 0,1,0], // :
];

fn glyph_index(c: char) -> Option<usize> {
    match c {
        '0'..='9' => Some(c as usize - '0' as usize),
        'A'..='Z' => Some(10 + c as usize - 'A' as usize),
        'a'..='z' => Some(10 + c as usize - 'a' as usize),
        '-' => Some(36),
        '+' => Some(37),
        '/' => Some(38),
        ':' => Some(39),
        _ => None,
    }
}

pub fn draw_text(frame: &mut [u8], width: usize, text: &str, x: usize, y: usize, color: [u8; 4], scale: f32) {
    for (i, c) in text.chars().enumerate() {
        if let Some(idx) = glyph_index(c) {
            draw_glyph(frame, width, idx, x + i * (CHAR_WIDTH*scale as usize + (SPACING*scale as usize).max(1)), y, color, scale as usize);
        }
    }
}

fn draw_glyph(frame: &mut [u8], width: usize, idx: usize, x: usize, y: usize, color: [u8; 4], scale: usize,) {
    for row in 0..CHAR_HEIGHT {
        for col in 0..CHAR_WIDTH {
            if GLYPHS[idx][row * CHAR_WIDTH + col] == 1 {
                // draw a scale×scale block instead of one pixel
                for sy in 0..scale {
                    for sx in 0..scale {
                        set_pixel(frame, width, x + col * scale + sx, y + row * scale + sy, color);
                    }
                }
            }
        }
    }
}

pub fn set_pixel(frame: &mut [u8], width: usize, x: usize, y: usize, color: [u8; 4]) {
    let i = (y * width + x) * 4;
    if i + 4 <= frame.len() {
        frame[i..i + 4].copy_from_slice(&color);
    }
}