use itertools::Itertools;
fn main() {
    let nums = include_str!("input")
                .split('\n')
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect::<Vec<i32>>();
    
    println!("Puzzle 1 result {}",solve1(&nums));
    println!("Puzzle 2 result {}",solve2(&nums));
}

fn solve1(nums : &[i32])->usize{
    nums.iter().tuple_windows().filter(|(x,y)| x<y).count()
}
fn solve2(nums : &[i32])->usize{
    nums.iter().tuple_windows().filter(|(p,_,_,z)| p<z).count()
}
