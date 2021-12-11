use std::collections::HashMap;

fn main() {
    let opening_to_closing: HashMap<char, char> =
        HashMap::from([('(', ')'), ('{', '}'), ('[', ']'), ('<', '>')]);
    let scores: HashMap<char, i32> =
        HashMap::from([(')', 3), ('}', 1197), (']', 57), ('>', 25137)]);

    let lines: Vec<Vec<char>> = std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    // Part 1
    let mut score = 0;
    for line in lines.iter() {
        let mut parens_stack = Vec::new();
        for ch in line {
            if opening_to_closing.contains_key(&ch) {
                parens_stack.push(ch);
            } else {
                if let Some(paren) = parens_stack.pop() {
                    if *ch != opening_to_closing[&paren] {
                        score += scores[&ch];
                    }
                }
            }
        }
    }

    println!("{}", score);

    // Part 2
    let scores: HashMap<char, usize> = HashMap::from([(')', 1), ('}', 3), (']', 2), ('>', 4)]);
    let mut lines_scores = Vec::new();
    'lines_loop: for line in lines.iter() {
        let mut parens_stack = Vec::new();
        for ch in line {
            if opening_to_closing.contains_key(&ch) {
                parens_stack.push(ch);
            } else {
                if let Some(paren) = parens_stack.pop() {
                    if *ch != opening_to_closing[&paren] {
                        continue 'lines_loop;
                    }
                }
            }
        }

        let mut score = 0usize;
        parens_stack.into_iter().rev().for_each(|ch| {
            score *= 5;
            score += scores[&opening_to_closing[ch]]
        });
        lines_scores.push(score);
    }

    lines_scores.sort();
    let middle_score = lines_scores[lines_scores.len() / 2];
    println!("{}", middle_score);
}
