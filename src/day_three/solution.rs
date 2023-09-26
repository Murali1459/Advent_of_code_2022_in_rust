pub fn part1(input: Vec<&str>) -> i32 {
    let mut common: Vec<char> = Vec::new();
    let mut score = 0;
    for j in input.into_iter() {
        let part1 = j[..(j.len() / 2)].to_string();
        let part2 = j[(j.len() / 2)..].to_string();
        for c in part1.chars() {
            if part2.contains(c) {
                common.push(c);
                break;
            }
        }
    }
    for j in common {
        score += get_points(j);
        // println!("{j} -> {}", get_points(j));
    }
    return score;
}

pub fn part2(input: Vec<&str>) -> i32 {
    let batches = input.chunks(3);
    let mut common: Vec<char> = Vec::new();
    let mut score = 0;
    for batch in batches {
        for j in ('a'..='z').chain('A'..='Z') {
            if batch[0].contains(j) && batch[1].contains(j) && batch[2].contains(j) {
                common.push(j);
                break;
            }
        }
    }
    for c in common {
        score += get_points(c);
        println!("{c} -> {}", get_points(c));
    }
    return score;
}

fn get_points(c: char) -> i32 {
    match c {
        'a'..='z' => return c as i32 - 'a' as i32 + 1,
        'A'..='Z' => return c as i32 - 'A' as i32 + 27,
        _ => {
            println!("found NOTHING");
            return 0;
        }
    }
}
