pub fn part1(input: Vec<&str>) -> i32 {
    let mut val = 0;
    for i in input.into_iter() {
        let groups: Vec<&str> = i.split(",").collect();
        let g1: &str = groups[0];
        let g2: &str = groups[1];
        let r1 = get_ranges(g1);
        let r2 = get_ranges(g2);
        val += is_full_overlapping(r1, r2) as i32;
    }
    return val;
}

pub fn part2(input: Vec<&str>) -> i32 {
    let mut val = 0;
    for i in input.into_iter() {
        let groups: Vec<&str> = i.split(",").collect();
        let g1: &str = groups[0];
        let g2: &str = groups[1];
        let r1 = get_ranges(g1);
        let r2 = get_ranges(g2);
        // if is_overlapping(r1, r2) {
        //     println!("{:?} {:?}", r1, r2);
        // };
        val += is_overlapping(r1, r2) as i32;
    }
    return val;
}

fn is_full_overlapping((l1, r1): (i32, i32), (l2, r2): (i32, i32)) -> bool {
    if l1 >= l2 && r1 <= r2 {
        // println!("{l1} >= {l2} && {r1} <= {r2}");
        return true;
    }
    if l2 >= l1 && r2 <= r1 {
        // println!("{l2} >= {l1} && {r2} <= {r1}");
        return true;
    }
    return false;
}

fn is_overlapping((l1, r1): (i32, i32), (l2, r2): (i32, i32)) -> bool {
    return l1 <= r2 && l2 <= r1;
}

fn get_ranges(range: &str) -> (i32, i32) {
    let range_array: Vec<&str> = range.split("-").collect();
    let left: i32 = range_array[0].parse().unwrap();
    let right: i32 = range_array[1].parse().unwrap();
    return (left, right);
}
