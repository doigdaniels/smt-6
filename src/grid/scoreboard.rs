pub struct Scoreboard {
    pub title: String,
    score: i32,
} impl Scoreboard {
    pub fn new(title: String) -> Self {
        Scoreboard {
            title: title,
            score: 0,
        }
    }

    pub fn inc(&mut self) {
        self.score += 1;
    }

    pub fn to_string(&self, length: usize) -> String {
        format!(" {title} {score:>len$}",
                title=self.title,
                score= format!("Score: {} ", self.score),
                len= length - self.title.len() - 1)
    }
}