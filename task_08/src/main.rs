use std::collections::HashMap;

fn contains(lhs: &String, rhs: &String) -> bool {
    let mut all_chars_found = true;
    rhs.chars().for_each(|rhs_ch| {
        if lhs.chars().filter(|lhs_ch| *lhs_ch == rhs_ch).count() == 0 {
            all_chars_found = false;
        }
    });
    all_chars_found
}

fn count_contains(lhs: &String, rhs: &String) -> i32 {
    let mut count = 0;
    rhs.chars().for_each(|rhs_ch| {
        if lhs.chars().filter(|lhs_ch| *lhs_ch == rhs_ch).count() > 0 {
            count += 1;
        }
    });
    count
}

fn determine_digit(digits_map: &HashMap<String, i32>, digit_str: &String) -> i32 {
    for (key, value) in digits_map {
        if key.len() == digit_str.len() && contains(key, digit_str) {
            return *value;
        }
    }
    -1
}

fn determine_value(digit_strings: Vec<String>, output_strings: Vec<String>) -> i32 {
    let get_by_len = |len| {
        digit_strings
            .iter()
            .filter_map(|x| {
                if x.len() == len {
                    Some(x.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .clone()
    };
    let get_by_len_and_contains = |len, cont| {
        digit_strings
            .iter()
            .filter_map(|x| {
                if x.len() == len && contains(x, cont) {
                    Some(x.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .clone()
    };
    let get_by_len_and_count_contains_and_is_not = |len, cont, cnt, oth| {
        digit_strings
            .iter()
            .filter_map(|x| {
                if x.len() == len && count_contains(x, cont) == cnt && !contains(x, oth) {
                    Some(x.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .clone()
    };

    //5-fbead 3-dcabe 2-bcega 0-gfbecd 7-ecd 4-dgac 1-cd 9-bedcag 8-agebcfd 6-fcagbe
    //ced cgbefad gbcaef cd
    let digit_1 = get_by_len(2);
    let digit_4 = get_by_len(4);
    let digit_7 = get_by_len(3);
    let digit_8 = get_by_len(7);
    let digit_9 = get_by_len_and_contains(6, &digit_4);
    let digit_3 = get_by_len_and_contains(5, &digit_7);
    let digit_0 = get_by_len_and_count_contains_and_is_not(6, &digit_1, 2, &digit_9);
    let digit_5 = digit_strings
        .iter()
        .filter_map(|x| {
            if x.len() == 5 && count_contains(x, &digit_4) == 3 && !contains(x, &digit_3) {
                Some(x.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .clone();
    let digit_6 = digit_strings
        .iter()
        .filter_map(|x| {
            if x.len() == 6 && !contains(x, &digit_9) && !contains(x, &digit_0) {
                Some(x.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .clone();
    let digit_2 = digit_strings
        .iter()
        .filter_map(|x| {
            if x.len() == 5 && !contains(x, &digit_3) && !contains(x, &digit_5) {
                Some(x.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .clone();

    let mut digits_map = HashMap::new();
    digits_map.insert(digit_0, 0);
    digits_map.insert(digit_1, 1);
    digits_map.insert(digit_2, 2);
    digits_map.insert(digit_3, 3);
    digits_map.insert(digit_4, 4);
    digits_map.insert(digit_5, 5);
    digits_map.insert(digit_6, 6);
    digits_map.insert(digit_7, 7);
    digits_map.insert(digit_8, 8);
    digits_map.insert(digit_9, 9);

    let output_digit_1 = determine_digit(&digits_map, &output_strings[0]);
    let output_digit_2 = determine_digit(&digits_map, &output_strings[1]);
    let output_digit_3 = determine_digit(&digits_map, &output_strings[2]);
    let output_digit_4 = determine_digit(&digits_map, &output_strings[3]);

    (output_digit_1 * 1000) + (output_digit_2 * 100) + (output_digit_3 * 10) + (output_digit_4)
}

fn main() {
    let lines: Vec<String> = std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
        .map(|x| String::from(x))
        .collect();

    // Part 1
    let selected_digs_count: usize = lines
        .iter()
        .map(|x| {
            x.split('|')
                .last()
                .unwrap()
                .split_whitespace()
                .filter(|seg| seg.len() == 2 || seg.len() == 4 || seg.len() == 3 || seg.len() == 7)
                .count()
        })
        .sum();
    println!("{}", selected_digs_count);

    // Part 2
    let mut sum = 0;
    for line in lines {
        let chunks: Vec<String> = line.split('|').map(|x| String::from(x)).collect();
        let input: Vec<String> = chunks[0]
            .split_whitespace()
            .map(|x| String::from(x))
            .collect();
        let output: Vec<String> = chunks[1]
            .split_whitespace()
            .map(|x| String::from(x))
            .collect();

        sum += determine_value(input, output);
    }

    println!("{}", sum);
}
