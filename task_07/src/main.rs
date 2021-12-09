fn main() {
    let lines: Vec<String> = std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
        .map(|x| String::from(x))
        .collect();
    let values: Vec<i32> = lines
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    // Part 1
    let mut fuels = vec![0; values.len()];
    for (i, value) in values.iter().enumerate() {
      fuels[i] = values.iter().map(|x| (x - value).abs()).sum();
    }
    println!("{}", fuels.iter().min().unwrap());

    // Part 2
    for (i, value) in values.iter().enumerate() {
      fuels[i] = values.iter().map(|x| {
        let distance = (x - value).abs();
        let a1 = 1;
        let an = distance;
        ((a1 + an) * distance) / 2
      }).sum();
    }
    println!("{}", fuels.iter().min().unwrap());
}
