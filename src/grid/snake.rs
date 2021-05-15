pub enum Dir {
    RIGHT,
    UP,
    LEFT,
    DOWN,
}

pub struct Snake {
    parts: Vec<(u32, u32)>,
    pub dir: Dir,
} impl Snake {
    pub fn new(x: i32, y: i32, dir: Dir) -> Self {
        Snake {
            parts: vec![(x as u32, y as u32), ],
            dir: dir,
        }
    }

    pub fn size(&self) -> i32 {
        self.parts.len() as i32
    }

    pub fn get_parts(&self) -> Vec<(u32, u32)> {
        self.parts.to_vec()
    }

    pub fn on_tick(&mut self) {
        match &self.dir {
            Dir::RIGHT => self.parts.insert(0, (self.parts[0].0 + 1, self.parts[0].1)),
            Dir::UP => self.parts.insert(0, (self.parts[0].0, self.parts[0].1 - 1)) ,
            Dir::LEFT => self.parts.insert(0, (self.parts[0].0 - 1, self.parts[0].1)),
            Dir::DOWN => self.parts.insert(0, (self.parts[0].0, self.parts[0].1 + 1)),
        }
    }

    pub fn pop(&mut self) {
        self.parts.pop();
    }
}
