use std::str::FromStr;

const BOARD_SIZE: usize = 5;
const CHUNK_SIZE: usize = BOARD_SIZE + 1;
struct BingoBoard {
    pub board: Vec<Vec<i32>>,
    hits: Vec<Vec<bool>>,
}

impl BingoBoard {
    fn new() -> BingoBoard {
        let board = Vec::new();
        let hits = vec![vec![false; BOARD_SIZE]; BOARD_SIZE];
        BingoBoard { board, hits }
    }

    fn try_hit(&mut self, number: i32) {
        self.board.iter().enumerate().for_each(|(x, row)| {
            row.iter().enumerate().for_each(|(y, val)| {
                if *val == number {
                    self.hits[x][y] = true
                }
            });
        });
    }

    fn check_if_winning(&self) -> bool {
        for x in 0..BOARD_SIZE {
            let mut is_winning = true;
            for y in 0..BOARD_SIZE {
                is_winning = is_winning && self.hits[x][y];
            }
            if is_winning {
                return true;
            }
        }
        for y in 0..BOARD_SIZE {
            let mut is_winning = true;
            for x in 0..BOARD_SIZE {
                is_winning = is_winning && self.hits[x][y];
            }
            if is_winning {
                return true;
            }
        }
        false
    }

    fn unmarked_sum(&self) -> i32 {
        let mut sum = 0;
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                if !self.hits[x][y] {
                    sum += self.board[x][y];
                }
            }
        }
        sum
    }
}

impl core::fmt::Debug for BingoBoard {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}\n", &self.board)
    }
}

fn main() {
    let lines: Vec<String> = std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
        .map(|x| String::from_str(x).unwrap())
        .collect();

    let hits: Vec<i32> = lines
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut boards = Vec::<BingoBoard>::new();
    lines
        .into_iter()
        .skip(2)
        .collect::<Vec<String>>()
        .chunks(CHUNK_SIZE)
        .enumerate()
        .for_each(|(i, chunk)| {
            boards.push(BingoBoard::new());
            chunk.iter().for_each(|x| {
                if x.len() > 0 {
                    boards[i].board.push(
                        x.split_whitespace()
                            .map(|val| val.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>(),
                    );
                }
            });
        });

    let mut indices_scores = Vec::<(usize, i32)>::new();
    for hit in hits {
        for (i, board) in boards.iter_mut().enumerate() {
            board.try_hit(hit);
            if board.check_if_winning() && !indices_scores.iter().any(|x| x.0 == (i)) {
                indices_scores.push((i, board.unmarked_sum() * hit));
            }
        }
    }

    println!(
        "First: {}, Last: {}",
        indices_scores.first().unwrap().1,
        indices_scores.last().unwrap().1
    )
}
