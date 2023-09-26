use super::game::{choose_to_draw, choose_to_lose, choose_to_win, winner, Game};

pub fn part1(input: Vec<&str>) -> i32 {
    let mut score: i32 = 0;
    for i in input.into_iter() {
        let choices: Vec<&str> = i.split(" ").collect();
        if choices.len() != 2 {
            continue;
        }
        let p1 = Game::create_enum(choices[0]);
        let p2 = Game::create_enum(choices[1]);
        score += p2.get_score();
        let winner = winner(p1, p2);
        if winner == 2 {
            score += 6;
        }
        if winner == 0 {
            score += 3;
        }
    }
    return score;
}
pub fn part2(input: Vec<&str>) -> i32 {
    let mut score: i32 = 0;
    for i in input.into_iter() {
        let choices: Vec<&str> = i.split(" ").collect();
        if choices.len() != 2 {
            continue;
        }
        let p1 = Game::create_enum(choices[0]);
        let pred = choices[1];
        let p2;
        if pred == "X" {
            p2 = choose_to_lose(p1);
        } else if pred == "Y" {
            p2 = choose_to_draw(p1);
            score += 3;
        } else {
            p2 = choose_to_win(p1);
            score += 6;
        }
        score += p2.get_score();
    }
    return score;
}
