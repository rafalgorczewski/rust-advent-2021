fn main() {
    let values: Vec<i32> = std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    // Part 1
    let mut previous = values[0];
    let mut counter = 0;
    values.iter().skip(1).for_each(|x| {
        if *x > previous {
            counter += 1;
        }
        previous = *x;
    });

    println!("{}", counter);

    // Part 2
    let mut previous = values.windows(3).next().unwrap().iter().sum::<i32>();
    let mut counter = 0;
    values.windows(3).skip(1).for_each(|x| {
        let sum = x.iter().sum::<i32>();
        if sum > previous {
            counter += 1;
        }
        previous = sum;
    });

    println!("{}", counter);
}
