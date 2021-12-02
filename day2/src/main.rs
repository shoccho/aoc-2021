#[derive(Debug)]
enum Command {
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl Command {
    fn parse(s: &str) -> Self {
        let words: Vec<&str> = s.trim().split_whitespace().collect();
        match words.as_slice() {
            ["forward", val] => (Command::Forward(val.parse().unwrap())),
            ["up", val] => (Command::Up(val.parse().unwrap())),
            ["down", val] => (Command::Down(val.parse().unwrap())),
            _ => panic!("This shouldn't happe"),
        }
    }
}

#[derive(Debug)]
struct Submarine {
    x: i64,
    y: i64,
    aim: i64,
}
impl Submarine {
    fn new() -> Submarine {
        Submarine { x: 0, y: 0, aim: 0 }
    }
    fn operate(&mut self, command: &Command) {
        match command {
            Command::Up(value) => self.y -= value,
            Command::Down(value) => self.y += value,
            Command::Forward(value) => self.x += value,
        }
    }
    fn operate_with_direction(&mut self, command: &Command) {
        match command {
            Command::Up(value) => self.aim -= value,
            Command::Down(value) => self.aim += value,
            Command::Forward(value) => {
                self.x += value;
                self.y += self.aim * value;
            }
        }
    }
}

fn main() {
    let mut submarine = Submarine::new();
    let mut submarine2 = Submarine::new();

    let commands = include_str!("input")
        .lines()
        .map(Command::parse)
        .collect::<Vec<Command>>();
    for command in commands {
        submarine.operate(&command);
        submarine2.operate_with_direction(&command);
    }
    println!("Puzzle 1: {}", submarine.x * submarine.y);
    println!("Puzzle 2: {}", submarine2.x * submarine2.y);
}
