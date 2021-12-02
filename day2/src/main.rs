fn main() {
    let lines = include_str!("input").lines().collect::<Vec<&str>>();

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
            y+= (aim * val);
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