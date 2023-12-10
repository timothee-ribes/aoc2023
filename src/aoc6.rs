
fn aoc6() {
    // Time:        54     70     82     75
    // Distance:   239   1142   1295   1253
    // let data = vec![(54, 239), (70, 1142), (82, 1295), (75, 1253)];

    // Time:      7  15   30
    // Distance:  9  40  200  
    // let data = vec![(7, 9), (15, 40), (30, 200)];
    let mut result = 1;

    // Time:      71530
    // Distance:  940200
    // let data = vec![(71530, 940200)];

    // Time:        54708275
    // Distance:   239114212951253
    let data:Vec<(u64, u64)> = vec![(54708275, 239114212951253)];

    for (duration, length) in data {
        let mut hold = 0;
        while hold * (duration - hold) <= length {
            hold += 1;
        }
        // hold += 1;
        println!("{}:{}:{}:{}", hold, duration, length, hold * (duration - hold));
        let win_count = duration - (2 * hold) + 1;
        result *= win_count;
        println!("{}", result);
    }
}
