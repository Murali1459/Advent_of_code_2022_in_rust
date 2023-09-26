pub fn day_one(input: Vec<&str>) -> i32 {
    let mut a: Vec<i32> = vec![];
    let mut score: i32 = 0;
    for j in input.into_iter() {
        if j == "" {
            a.push(score);
            score = 0;
        } else {
            score += match j.parse() {
                Ok(i) => i,
                Err(_) => 0,
            }
        }
    }
    a.sort();
    a.reverse();
    // a[0] for part1
    a[0] + a[1] + a[2] // part2
}
