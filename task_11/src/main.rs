const HEIGHT: usize = 10;
const WIDTH: usize = 10;

fn increase_all(powers: &mut Vec<Vec<u32>>) {
    for row in powers.iter_mut() {
        for power in row {
            *power += 1;
        }
    }
}

fn any_full(powers: &Vec<Vec<u32>>) -> bool {
    powers.iter().any(|x| x.iter().any(|pow| *pow > 9))
}

fn discharge_all_full(powers: &mut Vec<Vec<u32>>, already_discharged: &mut Vec<Vec<bool>>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let power = powers[y][x];
            if power > 9 {
                powers[y][x] = 0;
                if !already_discharged[y][x] {
                    already_discharged[y][x] = true;
                    if y > 0 {
                        powers[y - 1][x] += 1;
                        if x > 0 {
                            powers[y - 1][x - 1] += 1;
                        }
                        if x + 1 < WIDTH {
                            powers[y - 1][x + 1] += 1;
                        }
                    }
                    if y + 1 < HEIGHT {
                        powers[y + 1][x] += 1;
                        if x > 0 {
                            powers[y + 1][x - 1] += 1;
                        }
                        if x + 1 < WIDTH {
                            powers[y + 1][x + 1] += 1;
                        }
                    }
                    if x > 0 {
                        powers[y][x - 1] += 1;
                    }
                    if x + 1 < WIDTH {
                        powers[y][x + 1] += 1;
                    }
                }
            }
        }
    }
}

fn zero_discharged(powers: &mut Vec<Vec<u32>>, already_discharged: &Vec<Vec<bool>>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if already_discharged[y][x] {
                powers[y][x] = 0;
            }
        }
    }
}

fn count_discharges(already_discharged: &Vec<Vec<bool>>) -> usize {
    let mut discharges = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if already_discharged[y][x] {
                discharges += 1;
            }
        }
    }
    discharges
}

fn main() {
    let mut powers: Vec<Vec<u32>> = std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
        .map(|x| x.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let mut discharges_sum = 0;
    for i in 1..1000 {
        increase_all(&mut powers);

        let mut already_discharged = vec![vec![false; WIDTH]; HEIGHT];
        while any_full(&powers) {
            discharge_all_full(&mut powers, &mut already_discharged);
        }
        zero_discharged(&mut powers, &already_discharged);
        let discharges = count_discharges(&already_discharged);
        discharges_sum += discharges;

        if discharges == 100 {
            println!("{}", i);
        }
    }

    println!("{}", discharges_sum);
}
