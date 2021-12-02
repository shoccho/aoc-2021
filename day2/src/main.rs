#[derive(Debug)]
enum Command{
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl Command{
    fn parse(s:&str)->Self{
        let words :Vec<&str> = s.trim().split_whitespace().collect();
        match words.as_slice(){
            ["forward",val]=> (Command::Forward(val.parse().unwrap())),
            ["up",val]=> (Command::Up(val.parse().unwrap())),
            ["down",val]=> (Command::Down(val.parse().unwrap())),
            _ =>panic!("This shouldn't happe"),
        }
    }
}

#[derive(Debug)]
struct Submarine {
    x: i64,
    y: i64
}
impl Submarine{
    fn new()-> Submarine{
        Submarine{
            x:0,
            y:0
        }
    }
    fn operate(&mut self,command :Command){
        match command{
            Command::Up(value)=>{self.y-=value},
            Command::Down(value)=>{self.y+=value},
            Command::Forward(value)=>{self.x+=value},
        }
    }
}


fn main() {
    let lines = include_str!("input").lines().collect::<Vec<&str>>();
    let mut submarine = Submarine::new();

    let commands = include_str!("input").lines().map(Command::parse).collect::<Vec<Command>>();
    for command in commands{
        submarine.operate(command)
    }
    println!("{:?}",submarine.x * submarine.y );

    solve1(&lines);
    solve2(&lines);
}

fn solve2(lines:&[&str]){
    let mut x :i32=0;
    let mut y :i32=0;
    let mut aim :i32=0;
    for line in lines{
        let mut tokens = line.split(' ');
        let command =tokens.next().unwrap();
        let val = tokens.next().unwrap().parse::<i32>().unwrap();
        if command =="forward"{
            x+=val;
            y+= aim * val;
        }else if command =="up"{
            aim-=val;
        }else if command =="down"{
            aim+=val;
        }
    }
    println!("{}",x*y);
}
fn solve1(lines:&[&str]){
    let mut x :i32=0;
    let mut y :i32=0;
    for line in lines{
        let mut tokens = line.split(' ');
        let command =tokens.next().unwrap();
        let val = tokens.next().unwrap().parse::<i32>().unwrap();
        if command =="forward"{
            x+=val;
        }else if command =="up"{
            y-=val;
        }else if command =="down"{
            y+=val;
        }
    }
    println!("{}",x*y);
}