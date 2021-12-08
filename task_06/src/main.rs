const OLD_FISH_DAYS: usize = 6;
const NEW_FISH_DAYS: usize = OLD_FISH_DAYS + 2;
const MAX_DAYS: usize = NEW_FISH_DAYS + 1;

const DAYS: usize = 256;

fn main() {
    let sections: Vec<String> = std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
        .map(|x| String::from(x))
        .collect();
    let mut fishes = vec![0u64; MAX_DAYS];
    sections.first()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap()).for_each(|x| {
            fishes[x] += 1;
        });

    for _day in 0..DAYS {
        fishes.rotate_left(1);
        fishes[OLD_FISH_DAYS] += fishes[NEW_FISH_DAYS];
    }
    let count: u64 = fishes.iter().sum();

    println!("{}", count);
}
