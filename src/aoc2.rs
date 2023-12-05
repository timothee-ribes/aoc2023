
pub fn aoc2(ip: String) -> (i32, (i32, i32, i32)) {
    let mut rgb = (0, 0, 0);
    let split: Vec<String> = ip.split(":")
        .map(|x| String::from(x))
        .collect();
    let header = &split[0];
    let id:i32 = header[5..header.len()].parse().unwrap();
    let rolls: Vec<String> = split[1].clone().split(";")
        .map(|x| String::from(x))
        .collect();
    for roll in rolls {
        roll.split(",").for_each(|x| {
            let mut it = x.split(" ");
            it.next().unwrap();
            let count:i32 = it.next().unwrap().parse().unwrap();
            let color = it.next().unwrap();
            match color {
                "red" => {rgb.0 = rgb.0.max(count)},
                "green" => {rgb.1 = rgb.1.max(count)},
                "blue" => {rgb.2 = rgb.2.max(count)},
                _ => {}
            }
            ()
        });
    }
    (id, rgb)
}
