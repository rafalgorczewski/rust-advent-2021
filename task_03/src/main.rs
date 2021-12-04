fn get_bits_counts(lines: &Vec<String>) -> Vec<i32> {
    let word_length = lines[0].len();
    let mut counts = vec![0; word_length];
    lines.iter().for_each(|x| {
        x.chars().enumerate().for_each(|(i, ch)| {
            if ch == '1' {
                counts[i] += 1;
            } else if ch == '0' {
                counts[i] -= 1;
            }
        })
    });
    counts
}

fn oxygen_filter_out(lines: Vec<String>, index: usize, counts: i32) -> Vec<String> {
    let most_common = if counts > 0 { '1' } else { '0' };
    lines
        .into_iter()
        .filter(|x| {
            if counts != 0 {
                x.chars().nth(index).unwrap() == most_common
            } else {
                x.chars().nth(index).unwrap() == '1'
            }
        })
        .collect()
}

fn carbon_filter_out(lines: Vec<String>, index: usize, counts: i32) -> Vec<String> {
  let most_common = if counts > 0 { '1' } else { '0' };
  lines
      .into_iter()
      .filter(|x| {
          if counts != 0 {
              x.chars().nth(index).unwrap() != most_common
          } else {
              x.chars().nth(index).unwrap() == '0'
          }
      })
      .collect()
}

fn main() {
    let values: Vec<String> = std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
        .map(|x| String::from(x))
        .collect();

    let counts = get_bits_counts(&values);

    // Part 1
    let gamma = counts
        .iter()
        .map(|x| if *x > 0 { '1' } else { '0' })
        .collect::<String>();
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = counts
        .iter()
        .map(|x| if *x < 0 { '1' } else { '0' })
        .collect::<String>();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();

    println!("{}", gamma * epsilon);

    // Part 2
    let mut oxygen = values.clone();
    let mut carbon = values.clone();
    let word_length = values[0].len();
    for i in 0..word_length {
      if oxygen.len() > 1 {
        let oxygen_counts = get_bits_counts(&oxygen);
        oxygen = oxygen_filter_out(oxygen, i, oxygen_counts[i]);
      }
      if carbon.len() > 1 {
        let carbon_counts = get_bits_counts(&carbon);
        carbon = carbon_filter_out(carbon, i, carbon_counts[i]);
      }
    }

    let oxygen = usize::from_str_radix(&oxygen[0], 2).unwrap();
    let carbon = usize::from_str_radix(&carbon[0], 2).unwrap();

    println!("{}", oxygen * carbon);
}
