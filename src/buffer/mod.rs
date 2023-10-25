use crate::{C, H};
use std::usize;

pub struct ScrBuf {
    lines: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
    pub bg: char,
    pub rendered_frames: usize,
}

impl ScrBuf {
    pub fn new(w: usize, h: usize, bg: char) -> ScrBuf {
        ScrBuf {
            lines: vec![vec![bg; w]; h],
            width: w,
            height: h,
            bg,
            rendered_frames: 0,
        }
    }

    pub fn clear(&mut self, bg: char) -> () {
        self.bg = bg;
        self.lines = vec![vec![bg; self.width]; self.height];
    }

    pub fn set_cell(&mut self, x: usize, y: usize, chr: char) -> () {
        let line_idx = (self.height as isize) - 1 - (y as isize);
        self.lines[line_idx as usize][x] = chr;
    }

    pub fn render(&mut self) -> String {
        let mut acc: String = "".to_owned();
        let mut lines = self.lines.iter();
        while let Some(line) = lines.next() {
            let s_line: String = String::from_iter(line.iter());
            acc = acc + &s_line + "\n";
        }

        self.rendered_frames += 1;

        return acc;
    }

    pub fn blit(&mut self, x: usize, y: usize, s: &str) -> () {
        let characters = Vec::from_iter(s.chars());

        for i in 0..characters.len() {
            let pos = x + i;
            if pos >= self.width {
                continue;
            }

            self.set_cell(pos, y, characters[i]);
        }
    }

    pub fn r_blit(&mut self, x: usize, y: usize, s: &str) -> () {
        let characters = Vec::from_iter(s.chars());
        for i in 0..characters.len() {
            let pos = x - characters.len() + i;
            if pos >= self.width {
                continue;
            }

            self.set_cell(pos, y, characters[i]);
        }
    }

    pub fn blit_lines(&mut self, x: usize, y: usize, in_lines: &Vec<String>) {
        let lines: Vec<String> = Vec::from_iter(in_lines.iter().map(|s| s.clone()));
        let btm = (self.height as isize - (y as isize)) as usize;
        let top = (btm as isize - (lines.len() as isize)) as usize;

        for i in 0..lines.len() {
            let ly = (top as isize + (i as isize)) as usize;
            self.blit(x, ly, lines[lines.len() - 1 - i].as_str());
        }
    }

    pub fn frames(&self) -> usize {
        self.rendered_frames
    }
}

pub struct Counter {
    start: usize,
    stop: Option<usize>,
    step: isize,
    current: usize,
}

impl Counter {
    pub fn up(start: usize) -> Counter {
        return Counter {
            start,
            stop: None,
            step: 1,
            current: start,
        };
    }

    pub fn in_steps(start: usize, step: isize) -> Counter {
        Counter {
            start,
            stop: None,
            step,
            current: start,
        }
    }

    pub fn in_steps_to(start: usize, step: isize, stop: usize) -> Counter {
        Counter {
            start,
            stop: Some(stop),
            step,
            current: start,
        }
    }

    pub fn up_to(start: usize, count: usize) -> Counter {
        return Counter {
            start,
            stop: Some(start + count),
            step: 1,
            current: start,
        };
    }

    pub fn down_to(start: usize, stop: usize) -> Counter {
        Counter {
            start,
            stop: Some(stop),
            step: -1,
            current: start,
        }
    }

    pub fn reset(&mut self) -> () {
        self.current = self.start;
    }

    pub fn next(&mut self) -> usize {
        let next_val = (self.current as isize + self.step) as usize;
        let curr_val = self.current;
        if let Some(stop) = self.stop {
            if next_val >= stop {
                return stop;
            }
        }
        self.current = next_val;
        return curr_val;
    }
}

// ─│┌┐└┘├┤┬┴╶╵╷╴
const COMPACT_NUMBERS: [&str; 10 * C] = [
    "┌┐", "││", "└┘", " ┐", " │", " ┴", "╶┐", "┌┘", "└╴", "╶┐", "╶┤", "╶┘", "╷╷", "└┤", " ╵", "┌╴",
    "└┐", "╶┘", "┌╴", "├┐", "└┘", "╶┐", " │", " ╵", "┌┐", "├┤", "└┘", "┌┐", "└┤", "╶┘",
];

const NUMBERS: [&str; 10 * H] = [
    "┌─┐",
    "│ │",
    "│ │",
    "│ │",
    "└─┘",
    " ┐ ",
    " │ ",
    " │ ",
    " │ ",
    " ┴ ",
    "╶─┐",
    "  │",
    "┌─┘",
    "│  ",
    "└─╴",
    "╶─┐",
    "  │",
    "╶─┤",
    "  │",
    "╶─┘",
    "╷ ╷",
    "│ │",
    "└─┤",
    "  │",
    "  ╵",
    "┌─╴",
    "│  ",
    "└─┐",
    "  │",
    "╶─┘",
    "┌─╴",
    "│  ",
    "├─┐",
    "│ │",
    "└─┘",
    "╶─┐",
    "  │",
    "  │",
    "  │",
    "  ╵",
    "┌─┐",
    "│ │",
    "├─┤",
    "│ │",
    "└─┘",
    "┌─┐",
    "│ │",
    "└─┤",
    "  │",
    "╶─┘",
];
const DELIM: [&str; 3 * H] = [
    "   ", " • ", "   ", " • ", "   ", "   ", "   ", "   ", "   ", " • ", "   ", "   ", "   ",
    "   ", "   ",
];
const COMPACT_DELIM: [&str; 3 * C] = ["  ", " :", "  ", "  ", "  ", " •", "  ", "  ", "  "];

pub struct Symbols {
    pub chars: Vec<String>,
    pub delims: Vec<String>,
    pub width: usize,
    pub height: usize,
}

impl Symbols {
    pub fn normal() -> Symbols {
        Symbols {
            chars: Vec::from_iter(NUMBERS.map(|s| s.to_owned()).into_iter()),
            delims: Vec::from_iter(DELIM.map(|s| s.to_owned()).into_iter()),
            width: 3,
            height: 5,
        }
    }

    #[allow(dead_code)]
    pub fn compact() -> Symbols {
        Symbols {
            chars: Vec::from_iter(COMPACT_NUMBERS.map(|s| s.to_owned()).into_iter()),
            delims: Vec::from_iter(COMPACT_DELIM.map(|s| s.to_owned()).into_iter()),
            width: 2,
            height: 3,
        }
    }

    pub fn delimiters_of(&self, delim_char: char) -> Vec<String> {
        let offset: usize = match delim_char {
            ':' => 0,
            '.' => 1,
            _ => 2,
        };
        let start = offset * self.height;
        return Vec::from_iter(
            self.delims[start..start + self.height]
                .iter()
                .map(|s| s.clone()),
        );
    }

    pub fn nums_of(&self, i: usize) -> Vec<String> {
        let dig = i % 10;
        let offset = dig * self.height;
        return Vec::from_iter(
            self.chars[offset..offset + self.height]
                .iter()
                .map(|s| s.clone()),
        );
    }
}
