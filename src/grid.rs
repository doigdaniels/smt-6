mod snake;
mod scoreboard;

use {
    snake::{Snake, Dir,},
    ndarray::{Array, Ix2},
    scoreboard::Scoreboard,
    crossterm::event::{KeyCode, Event, KeyEvent},
    rand::{Rng, thread_rng,},
};

pub struct Texture {
    pub EMPTY: char,
    pub PLAYER: char,
    pub APPLE: char,
}

struct Apple(u32, u32);

pub struct Grid {
    a: Array<i32, Ix2>,
    snake: Snake,
    sb: Scoreboard,
    apple: Apple,
    tex: Texture,
} impl Grid {
    pub fn new(x: i32, y: i32, title: &str, tex: Texture) -> Self {
        Grid {
            a: Array::<i32, Ix2>::zeros((y.wrapping_abs() as u32 as usize,
                                         x.wrapping_abs() as u32 as usize)),
            snake: Snake::new(3, 5, Dir::RIGHT),
            sb: Scoreboard::new(String::from(title)),
            apple: Apple(7,5),
            tex: tex,
        }
    }

    pub fn parse_key(&mut self, ev: Event) -> bool {
        match self.snake.dir {
            Dir::RIGHT | Dir::LEFT => {
                match ev {
                    Event::Key(KeyEvent { code: KeyCode::Char('x'), .. }) => true,
                    Event::Key(KeyEvent { code: KeyCode::Char('w'), .. })
                    | Event::Key(KeyEvent { code: KeyCode::Up, .. })
                    => {self.snake.dir = Dir::UP; false },
                    Event::Key(KeyEvent { code: KeyCode::Char('s'), .. })
                    | Event::Key(KeyEvent { code: KeyCode::Down, .. })
                    => {self.snake.dir = Dir::DOWN;false},
                    _ => false,
                }
            },
            _ => {
                match ev {
                    Event::Key(KeyEvent { code: KeyCode::Char('x'), .. }) => true,
                    Event::Key(KeyEvent { code: KeyCode::Char('d'), .. })
                    | Event::Key(KeyEvent { code: KeyCode::Right, .. })
                    => {self.snake.dir = Dir::RIGHT; false},
                    Event::Key(KeyEvent { code: KeyCode::Char('a'), .. })
                    | Event::Key(KeyEvent { code: KeyCode::Left, .. })
                    => {self.snake.dir = Dir::LEFT; false},
                    _ => false,
                }
            },
        }
    }

    fn spawn_apple(&mut self) {
        let mut rng = thread_rng();
        loop {
            let x: usize = rng.gen_range(0..self.a.dim().1);
            let y: usize = rng.gen_range(0..self.a.dim().0);
            if self.a[[x, y]] == 0 {
                self.apple = Apple(x as u32, y as u32);
                break;
            }
        }
    }

    pub fn run_tick(&mut self) {
        self.a = Array::<i32, Ix2>::zeros(self.a.dim());
        self.snake.on_tick();

        self.a[[self.apple.0 as usize, self.apple.1 as usize]] = 2;
        for i in self.snake.get_parts() {
            self.a[[i.1 as usize, i.0 as usize]] = 1;
        }

        if self.snake.get_parts()[0].0 == self.apple.1 &&
                self.snake.get_parts()[0].1 == self.apple.0 {
            self.spawn_apple();
            self.sb.inc();
        } else {
            self.snake.pop();
        }

        if self.snake.get_parts()[1..].contains(&self.snake.get_parts()[0]) {
            panic!("Poo");
        }
    }

    pub fn to_iter(&self) -> Vec<String> {
        let mut map:Vec<String> = vec![];

        for i in self.a.outer_iter() {
            let mut str = String::from("");
            for j in i.iter() {
                str.push(' ');
                str.push(
                    match j {
                        1 => self.tex.PLAYER,
                        2 => self.tex.APPLE,
                        _ => self.tex.EMPTY,
                    }
                );
                str.push(' ');
            }
            map.push(str);
        }
        map.push(self.sb.to_string(map[0].len() - 1));

        map
    }
}