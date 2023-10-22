use std::string::String;
use std::vec::Vec;
use std::{thread, time, usize};


const H: usize = 5;

const numbers: [&str; 10*H] = [
        "┌─┐", 
        "│ │", 
        "│ │", 
        "│ │", 
        "└─┘",
        " ┐ ", 
        " │ ", 
        " │ ", 
        " │ ", 
        " ╵ ",
        "┌─┐", 
        "  │", 
        "┌─┘", 
        "│  ", 
        "└─╴",
        "┌─┐", 
        "  │", 
        " ─┤", 
        "  │", 
        "└─┘",
        "╷ ╷", 
        "│ │", 
        "└─┤", 
        "  │", 
        "  ╵",
        "┌─╴", 
        "│  ", 
        "└─┐", 
        "  │", 
        "└─┘",
        "┌─┐", 
        "│  ", 
        "├─┐", 
        "│ │", 
        "└─┘",
        "┌─┐", 
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
        "└─┘",
    ];

struct CellSpec {
    pub x: usize,
    pub y: usize,
    pub val: char,
}

#[allow(dead_code)]
impl CellSpec {
    fn default() -> CellSpec {
        CellSpec {
            x: 0,
            y: 0,
            val: ' ',
        }
    }
}

struct CellSpecs {
    x: usize,
    y: usize,
    cells: Vec<CellSpec>,
}

#[allow(dead_code)]
impl CellSpecs {
    fn resolve_cells(&self) -> Vec<CellSpec> {
        let mut result: Vec<CellSpec> = vec![];
        for spec in &self.cells {
            result.push(CellSpec {
                x: spec.x + self.x,
                y: spec.y + self.y,
                val: spec.val,
            });
        }

        return result;
    }
}

#[allow(dead_code)]
struct ScrBuf {
    lines: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
    pub bg: char,
    pub rendered_frames: usize,
}

#[allow(dead_code)]
impl ScrBuf {
    fn new(w: usize, h: usize, bg: char) -> ScrBuf {
        ScrBuf {
            lines: vec![vec![bg; w]; h],
            width: w,
            height: h,
            bg,
            rendered_frames: 0,
        }
    }

    fn clear(&mut self, bg: char) -> () {
        self.bg = bg as char;
        self.lines = vec![vec![bg; self.width]; self.height];
    }

    fn set_cell(&mut self, x: usize, y: usize, chr: char) -> () {
        self.lines[self.height - 1 - y][x] = chr;
    }

    fn render(&mut self) -> String {
        let mut acc: String = "".to_owned();
        let mut lines = self.lines.iter();
        while let Some(line) = lines.next() {
            let s_line: String = String::from_iter(line.iter());
            acc = acc + &s_line + "\n";
        }

        self.rendered_frames += 1;

        return acc;
    }

    fn blit(&mut self, x: usize, y: usize, s: &str) -> () {
        let characters = Vec::from_iter(s.chars());

        for i in 0..characters.len() {
            let pos = x + i;
            if pos >= self.width {
                continue;
            }

            self.set_cell(pos, y, characters[i]);
        }
    }

    fn r_blit(&mut self, x: usize, y: usize, s: &str) -> () {
        let characters = Vec::from_iter(s.chars());
        for i in 0..characters.len() {
            let pos = x - characters.len() + i;
            if pos >= self.width {
                continue;
            }

            self.set_cell(pos, y, characters[i]);
        }
    }

    fn specify_cells(&mut self, specs: Vec<CellSpec>) -> () {
        for spec in specs {
            self.set_cell(spec.x, spec.y, spec.val);
        }
    }

    fn frames(&self) -> usize {
        self.rendered_frames
    }
}

#[allow(dead_code)]
fn wiggle(dt: time::Duration, duration: time::Duration) -> isize {
    let t = dt.as_secs_f32() / duration.as_secs_f32();
    return (10.0 * t.sin()) as isize;
}

#[allow(dead_code)]
fn blink(s: &str, dt: time::Duration) -> String {
    let is_blinking = dt.as_millis() % 500 >= 400;
    let owned = String::from(s);
    if is_blinking {
        let blunk = owned.replace("@", "-");
        return blunk;
    }

    return owned;
}

struct Counter {
    start: usize,
    stop: Option<usize>,
    step: isize,
    current: usize,
}

#[allow(dead_code)]
impl Counter {
    fn up(start: usize) -> Counter {
        return Counter {
            start,
            stop: None,
            step: 1,
            current: start,
        };
    }

    fn up_to(start: usize, count: usize) -> Counter {
        return Counter {
            start,
            stop: Some(start + count),
            step: 1,
            current: start,
        };
    }

    fn down_to(start: usize, stop: usize) -> Counter {
        Counter {
            start,
            stop: Some(stop),
            step: -1,
            current: start,
        }
    }

    fn reset(&mut self) -> () {
        self.current = self.start;
    }

    fn next(&mut self) -> Option<usize> {
        let next_val = (self.current as isize + self.step) as usize;
        if let Some(stop) = self.stop {
            if next_val >= stop {
                return None;
            }
        }
        self.current = next_val;
        return Some(self.current);
    }
}

fn numeral(mut buf: ScrBuf, num: usize, x: usize, y: usize) -> ScrBuf {
    let valid_num = num % 10;
    let offset = valid_num * H;
    let strings = &numbers[offset..offset+H];
    let btm = buf.height - y;
    let top = btm - H;

    for i in 0..H {
        let ly = top - i - 1;
        buf.blit(x, ly, strings[i]);
    }

    return buf;
}

#[allow(dead_code)]
fn digit(base: usize, place: isize, num: usize) -> usize {
    let b = base as isize;
    let n = num as isize;
    let p = place as isize;

    let shifted: isize = n / b.pow(p as u32);
    let dig = shifted % b;
    return dig as usize;
}

fn main() {
    let mut buf = ScrBuf::new(30, 10, ' ');
    let fps: f64 = 10.0;
    let frame_time = time::Duration::from_nanos((1e9 / fps) as u64);
    let mut delta_t: time::Duration = time::Duration::ZERO;

    loop {
        buf.clear('.');
        buf = numeral(buf, (delta_t.as_secs() as usize) % 10, 3, 0);
        buf = numeral(buf, (delta_t.as_millis() / 100) as usize, 7, 0);
        println!("{}", buf.render());
        println!("{:?}", delta_t);
        thread::sleep(frame_time);
        delta_t = delta_t + frame_time;

        std::process::Command::new("clear").status().unwrap();
    }
}
