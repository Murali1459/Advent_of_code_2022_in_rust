#[derive(PartialEq, Eq)]
pub enum Game {
    Rock,
    Paper,
    Scissor,
}

impl Game {
    pub fn create_enum(a: &str) -> &Self {
        if a == "A" || a == "X" {
            return &Game::Rock;
        }
        if a == "B" || a == "Y" {
            return &Game::Paper;
        }
        return &Game::Scissor;
    }
    pub fn get_score(&self) -> i32 {
        match self {
            Game::Rock => 1,
            Game::Paper => 2,
            Game::Scissor => 3,
        }
    }
}

pub fn winner(p1: &Game, p2: &Game) -> i32 {
    if p1 == p2 {
        return 0;
    };
    if let (Game::Rock, Game::Paper) = (p1, p2) {
        return 2;
    }
    if let (Game::Paper, Game::Scissor) = (p1, p2) {
        return 2;
    }
    if let (Game::Scissor, Game::Rock) = (p1, p2) {
        return 2;
    }
    return 1;
}

pub fn choose_to_win(p1: &Game) -> &Game {
    if let Game::Rock = p1 {
        return &Game::Paper;
    }
    if let Game::Paper = p1 {
        return &Game::Scissor;
    }
    return &Game::Rock;
}

pub fn choose_to_draw(p1: &Game) -> &Game {
    return &p1;
}

pub fn choose_to_lose(p1: &Game) -> &Game {
    if let Game::Rock = p1 {
        return &Game::Scissor;
    }
    if let Game::Paper = p1 {
        return &Game::Rock;
    }
    return &Game::Paper;
}
