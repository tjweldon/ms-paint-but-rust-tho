pub mod buffer;

use buffer::{Counter, ScrBuf, Symbols};
use chrono::{Local, Timelike};
use std::string::String;
use std::{thread, time, usize};

const H: usize = 5;
const C: usize = 3;

fn numeral(buf: &mut ScrBuf, num: usize, x: usize, y: usize, symbols: &Symbols) {
    let valid_num = num % 10;
    let strings: Vec<String> = symbols.nums_of(valid_num);
    buf.blit_lines(x, y, &strings);
}

fn delim(buf: &mut ScrBuf, delimiter: char, x: usize, y: usize, symbols: &Symbols) {
    let strings: Vec<String> = symbols.delimiters_of(delimiter);
    buf.blit_lines(x, y, &strings);
}

fn digit(base: usize, place: isize, num: usize) -> usize {
    let b = base as isize;
    let n = num as isize;
    let p = place as u32;

    let shifted: isize = n / b.pow(p);
    let dig = shifted % b;
    return dig as usize;
}

fn main() {
    let mut buf = ScrBuf::new(50, 10, ' ');
    let fps: f64 = 24.0;

    let frame_time = time::Duration::from_nanos((1e9 / fps) as u64);
    std::process::Command::new("clear").status().unwrap();

    let symbols: Symbols = Symbols::normal();
    let mut btm: usize;

    let mut x_cursor;

    let mut dt = Local::now();
    let mut hour = dt.hour() as usize;
    let mut minute = dt.minute() as usize;
    let mut second = dt.second() as usize;
    let mut milli = dt.timestamp_subsec_millis() as usize;
    let mut mid_frame: time::Instant;
    let mut render_duration: time::Duration;
    let mut frame_start = time::Instant::now();
    let mut frame_over = frame_start.checked_add(frame_time).unwrap();
    let step = symbols.width as isize;
    loop {
        buf.clear(' ');
        btm = 0;
        x_cursor = Counter::in_steps(0, step);

        numeral(&mut buf, digit(10, 1, hour), x_cursor.next(), btm, &symbols);
        numeral(&mut buf, digit(10, 0, hour), x_cursor.next(), btm, &symbols);
        delim(&mut buf, ':', x_cursor.next(), btm, &symbols);

        numeral(
            &mut buf,
            digit(10, 1, minute),
            x_cursor.next(),
            btm,
            &symbols,
        );
        numeral(
            &mut buf,
            digit(10, 0, minute),
            x_cursor.next(),
            btm,
            &symbols,
        );
        delim(&mut buf, ':', x_cursor.next(), btm, &symbols);

        numeral(
            &mut buf,
            digit(10, 1, second),
            x_cursor.next(),
            btm,
            &symbols,
        );
        numeral(
            &mut buf,
            digit(10, 0, second),
            x_cursor.next(),
            btm,
            &symbols,
        );
        delim(&mut buf, '.', x_cursor.next(), btm, &symbols);
        numeral(
            &mut buf,
            digit(10, 2, milli),
            x_cursor.next(),
            btm,
            &symbols,
        );
        numeral(
            &mut buf,
            digit(10, 1, milli),
            x_cursor.next(),
            btm,
            &symbols,
        );

        print!("{}", buf.render());
        mid_frame = time::Instant::now();
        render_duration = mid_frame.duration_since(frame_start);
        println!("{:?}", render_duration);

        thread::sleep(frame_over.duration_since(time::Instant::now()));
        frame_start = time::Instant::now();
        frame_over = frame_start.checked_add(frame_time).unwrap();

        dt = Local::now();
        milli = dt.timestamp_subsec_millis() as usize;
        second = dt.second() as usize;
        minute = dt.minute() as usize;
        hour = dt.hour() as usize;

        std::process::Command::new("clear").status().unwrap();
    }
}
