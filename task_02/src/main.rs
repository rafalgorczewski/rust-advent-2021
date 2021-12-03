fn main() {
    // Part 1
    let mut depth = 0;
    let mut horizontal = 0;
    std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
        .for_each(|x| {
            let words = x.trim().split_whitespace().collect::<Vec<&str>>();
            let (direction, value) = (words[0], words[1].parse::<i32>().unwrap());
            match direction {
                "forward" => horizontal += value,
                "up" => depth -= value,
                "down" => depth += value,
                _ => (),
            };
        });

    println!("{}", depth * horizontal);

    // Part 2
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal = 0;
    std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
        .for_each(|x| {
            let words = x.trim().split_whitespace().collect::<Vec<&str>>();
            let (direction, value) = (words[0], words[1].parse::<i32>().unwrap());
            match direction {
                "forward" => {
                    horizontal += value;
                    depth += aim * value;
                },
                "up" => aim -= value,
                "down" => aim += value,
                _ => (),
            };
        });

    println!("{}", depth * horizontal);
}
