const SIZE: usize = 5;
const ALL_MARKED: i32 = -5;

#[derive(Debug)]
struct Board {
    nums: Vec<Vec<i32>>,
    won: bool,
}
impl Board {
    fn from_str(chunk: &str) -> Self {
        Board {
            nums: chunk
                .split('\n')
                .into_iter()
                .map(|x| {
                    x.split_whitespace()
                        .map(|x| -> i32 { x.parse().unwrap() })
                        .collect::<Vec<i32>>()
                })
                .collect(),
            won: false,
        }
    }
    fn mark(&mut self, num: i32) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.nums[i][j] == num {
                    self.nums[i][j] = -1;
                }
            }
        }
    }
    pub fn sum(&self) -> i32 {
        self.nums
            .iter()
            .flat_map(|r| r.iter().filter(|s| **s != -1))
            .sum()
    }

    fn check(&self) -> bool {
        for i in 0..self.nums.len() {
            if self.nums[i].iter().sum::<i32>() == ALL_MARKED {
                return true;
            }
            let mut col_sum: i32 = 0;
            for j in 0..SIZE {
                col_sum += self.nums[j][i];
            }
            if col_sum == ALL_MARKED {
                return true;
            }
        }
        false
    }
}
fn main() {
    let (input, data) = include_str!("input").split_once("\n\n").unwrap();

    let numbers = input
        .split(',')
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect::<Vec<i32>>();

    let mut boards: Vec<Board> = data.split("\n\n").map(|s| Board::from_str(s)).collect();

    solve1and2(numbers, &mut boards);
}

fn solve1and2(numbers: Vec<i32>, boards: &mut Vec<Board>) {
    let mut count = 0;
    let len = boards.len();
    for number in numbers {
        *boards = boards.drain(..).filter(|x| !x.won).collect(); 

        for board in &mut *boards {
            board.mark(number);
            if board.check() {
                board.won = true;
                if count == 0 || count == len - 1 {
                    println!("{:?}", board.sum() * number);
                }
                count += 1;
            }
        }
    }
}
