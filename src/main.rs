mod grid;

use {
    grid::{Grid, Texture},
    crossterm::{
        execute,
        event::{poll, read, Event, KeyCode, KeyEvent},
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode,
        Clear, ClearType},
        style::Print,
        cursor,
    },
    std::{
        thread,
        time::Duration,
        io::{Write, stdout},
    },
};

const TICK_SPEED: Duration = Duration::from_millis(300);

const TITLE: &str = "SMT 5 ('x' to quit)";

const WIDTH: i32 = 15;
const HEIGHT: i32 = 15;

const TEX: Texture = Texture {
    EMPTY: '.',
    PLAYER: '#',
    APPLE: 'o',
};

fn add_borders(ls: Vec<String>) -> String {
    let mut map = String::from("");

    for i in 0..ls[0].len() + 2 {
        map.push('-');
    }
    map.push('\n');

    for str in ls.iter() {
        map.push_str(&*format!("|{}|\n", str));
    }

    for i in 0..ls[0].len() + 2 {
        map.push('-');
    }
    map.push('\n');

    map
}

fn main() {
    let mut grid = Grid::new(WIDTH, HEIGHT, TITLE, TEX);

    execute!(stdout(), EnterAlternateScreen);

    loop {
        grid.run_tick();
        execute!(stdout(), Print(add_borders(grid.to_iter())));
        enable_raw_mode();
        if poll(TICK_SPEED).unwrap() {
            let event = read().unwrap();
            if grid.parse_key(event) {
                break;
            }
        }
        disable_raw_mode();
        execute!(stdout(), Clear(ClearType::All));
    }

    disable_raw_mode();
    execute!(stdout(), LeaveAlternateScreen);
}
