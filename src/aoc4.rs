pub fn card_count(scores: &Vec<u32>, i: usize) -> i32 {
    let mut count = 1;
    let wins = scores[i];
    for j in i + 1..i + (wins as usize) + 1 {
        count += card_count(scores, j);
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::aoc4::aoc4;

    #[test]
    fn it_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let result = aoc4(input.to_string());
        assert_eq!(result, 4);
    }
    #[test]
    fn two_zero_two() {
        let input = "Card 203: 37 80  7 87 79 60  6 49 16 12 | 23 34 15 46 38 20 27 45 33 97 37 14 68 83 49 79 43 70 57 60 11 63 24 35 73";
        let result = aoc4(input.to_string());
        assert_eq!(result, 0);
    }
}

pub fn aoc4(ip: String) -> u32 {
    let split: Vec<String> = ip
        .split(|x| x == ':' || x == '|')
        .map(|x| String::from(x))
        .collect();

    let winning_numbers: Vec<_> = split[1].split_whitespace().collect();

    let mut win_count = 0;
    split[2].split_whitespace().for_each(|x| {
        if winning_numbers.contains(&x) {
            win_count += 1;
        }
    });
    win_count
}
