use std::collections::{HashSet, VecDeque};

fn flood_basin(
    first_pos: (usize, usize),
    heights: &Vec<Vec<i32>>,
    heights_flooded: &mut Vec<Vec<bool>>,
) -> usize {
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut to_visit = VecDeque::<(usize, usize)>::new();
    to_visit.push_back(first_pos);

    let mut count = 0;
    while !to_visit.is_empty() {
        let (y, x) = to_visit.pop_front().unwrap();
        if !visited.contains(&(y, x)) {
            if y > 0 {
                let target_pos = (y - 1, x);
                if heights[target_pos.0][target_pos.1] != 9 && !visited.contains(&target_pos) {
                    to_visit.push_back(target_pos);
                }
            }
            if y + 1 < heights.len() {
                let target_pos = (y + 1, x);
                if heights[target_pos.0][target_pos.1] != 9 && !visited.contains(&target_pos) {
                    to_visit.push_back(target_pos);
                }
            }
            if x > 0 {
                let target_pos = (y, x - 1);
                if heights[target_pos.0][target_pos.1] != 9 && !visited.contains(&target_pos) {
                    to_visit.push_back(target_pos);
                }
            }
            if x + 1 < heights.first().unwrap().len() {
                let target_pos = (y, x + 1);
                if heights[target_pos.0][target_pos.1] != 9 && !visited.contains(&target_pos) {
                    to_visit.push_back(target_pos);
                }
            }

            count += 1;
            heights_flooded[y][x] = true;
            visited.insert((y, x));
        }
    }

    count
}

fn main() {
    let heights: Vec<Vec<i32>> = std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
        .map(|x| {
            x.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    // Part 1
    let mut lows = Vec::new();
    for (y, row) in heights.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            let mut is_lower = true;
            if y > 0 {
                is_lower = is_lower && *height < heights[y - 1][x];
            }
            if y + 1 < heights.len() {
                is_lower = is_lower && *height < heights[y + 1][x];
            }
            if x > 0 {
                is_lower = is_lower && *height < heights[y][x - 1];
            }
            if x + 1 < heights.first().unwrap().len() {
                is_lower = is_lower && *height < heights[y][x + 1];
            }

            if is_lower {
                lows.push(height + 1);
            }
        }
    }
    let risk: i32 = lows.iter().sum();
    println!("{}", risk);

    // Part 2
    let mut heights_flooded = vec![vec![false; heights.first().unwrap().len()]; heights.len()];
    let mut sizes = Vec::<usize>::new();

    for (y, row) in heights.iter().enumerate() {
        for (x, _height) in row.iter().enumerate() {
            if heights[y][x] != 9 && !heights_flooded[y][x] {
                let size = flood_basin((y, x), &heights, &mut heights_flooded);
                sizes.push(size);
            }
        }
    }

    sizes.sort();
    let sizes_product = sizes[sizes.len() - 1] * sizes[sizes.len() - 2] * sizes[sizes.len() - 3];
    println!("{}", sizes_product);
}
