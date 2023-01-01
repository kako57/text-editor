use ncurses::*;
use ropey::Rope;
use std::cmp::{max, min};
use std::env;
use std::error::Error;

type Cursor = (i32, i32);

fn num_digits(num: usize) -> usize {
    let mut digits = 0;
    let mut n = num;
    while n > 0 {
        n /= 10;
        digits += 1;
    }
    digits
}

fn print_buffer(buffer: &Rope, cursor: &mut Cursor, line_number_width: usize) {
    // get the terminal size
    let mut y = 0;
    let mut x = 0;
    getmaxyx(stdscr(), &mut y, &mut x);

    // get number of lines in file
    let num_lines = buffer.len_lines();

    // check if cursor is out of bounds
    if cursor.0 < 0 {
        cursor.0 = 0;
    } else if cursor.0 >= num_lines as i32 {
        cursor.0 = num_lines as i32 - 1;
    }

    let line = buffer.line(cursor.0 as usize);
    let line_len = line.len_chars();
    if cursor.1 < 0 {
        cursor.1 = 0;
    } else if cursor.1 >= line_len as i32 + line_number_width as i32 {
        cursor.1 = max(
            line_number_width as i32 + 1,
            line_len as i32 + line_number_width as i32 - 1,
        );
    }

    // print the buffer
    for i in 0..min(y, num_lines as i32) {
        let line = buffer.line(i as usize);
        mvaddstr(i, 0, format!("{:1$} ", i + 1, line_number_width).as_str());
        // limit the line to the terminal width

        mvaddstr(
            i,
            line_number_width as i32 + 1,
            line.slice(..min(x as usize, line.len_chars()))
                .as_str()
                .unwrap(),
        );
    }

    mv(cursor.0, cursor.1);
    refresh();
}

fn main() -> Result<(), Box<dyn Error>> {
    initscr();
    noecho();
    // curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // get the filename from the command line
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // create a rope to hold the file contents
    let buffer = Rope::from_reader(std::fs::File::open(filename).unwrap()).unwrap();

    // set cursor to top left, and save this position

    let mut min_cursor_x = num_digits(buffer.len_lines());
    let mut cursor = (0, min_cursor_x as i32 + 1);
    mv(cursor.0, cursor.1);

    // do an initial print
    print_buffer(&buffer, &mut cursor, min_cursor_x);

    // watch for change in terminal size
    let mut ch = ' ' as i32;
    while ch != 'q' as i32 {
        ch = getch();
        match ch as u8 as char {
            // hjkl for movement
            'h' => {
                cursor.1 -= 1;
            }
            'l' => {
                cursor.1 += 1;
            }
            'k' => {
                cursor.0 -= 1;
            }
            'j' => {
                cursor.0 += 1;
            }
            _ => {}
        }
        min_cursor_x = num_digits(buffer.len_lines());
        if cursor.1 <= min_cursor_x as i32 {
            cursor.1 = min_cursor_x as i32 + 1;
        }
        if cursor.0 < 0 {
            cursor.0 = 0;
        } else if cursor.0 >= buffer.len_lines() as i32 {
            cursor.0 = buffer.len_lines() as i32 - 1;
        }
        print_buffer(&buffer, &mut cursor, min_cursor_x);
    }
    endwin();

    Ok(())
}
