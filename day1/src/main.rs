fn main() {
    let nums = include_str!("input")
                .split('\n')
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect::<Vec<i32>>();
    
    println!("Puzzle 1 result {}",solve(&nums,1));
    println!("Puzzle 2 result {}",solve(&nums,3));
}
fn solve(nums : &[i32],gap:usize)->i32{
    
    let mut count = 0;
    for i in gap..nums.len(){
        if nums[i] > nums[i-gap]{
            count+=1
        }
    }
    count
    
}
