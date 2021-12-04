use itertools::Itertools;
const SIZE: usize = 5;
const NULL: i32 = -5;

#[derive(Debug)]
struct Board {
    nums : [[i32;SIZE];SIZE]
}
impl Board {
    fn new()->Self{
        Board{
            nums :[[0;SIZE];SIZE]
        }
    }
    fn from_str(chunk :Vec<&str>)->Self{
        let mut b = Board::new();
        for i in 0..chunk.len(){
            let nums = chunk[i].split_whitespace().map(|x|->i32{x.parse().unwrap()}).collect::<Vec<i32>>();
            for j in 0..nums.len(){
                b.nums[i][j]=nums[j];
            }
        }
        b
    }
    fn mark(&mut self,num: i32){
        for i in 0..SIZE{
            for  j in 0..SIZE{
                if self.nums[i][j]==num{
                    self.nums[i][j]=-1;
                }
            }
        }
    }
    fn sum(&self)->i32{
        let mut sum=0;
        for i in 0..SIZE{
            for  j in 0..SIZE{
                if self.nums[i][j]!=-1{
                    sum+=self.nums[i][j];
                }
            }
        }
        sum
    }
    fn check(&self)->bool{
        for i in 0..self.nums.len(){
            if self.nums[i].iter().sum::<i32>() == NULL {
                return true;
            }
            let mut col_sum:i32 = 0;
            for j in 0..SIZE{
                col_sum += self.nums[j][i];
            }
            if col_sum == NULL{
                return true;
            }
        }
        false
    }
}
fn main() {
   
    solve1();
}

fn solve1(){
     let mut lines = include_str!("input").lines().collect::<Vec<&str>>();
    let numbers = lines.first().unwrap().split(',').map(|x|->i32{x.parse().unwrap()}).collect::<Vec<i32>>();
    lines.drain(0..1);
    let mut boards :Vec<Board> = lines.into_iter().tuples().into_iter().map(|(_,a,b,c,d,e)| Board::from_str(vec![a,b,c,d,e]) ).collect();
    // dbg!(boards);
    // todo!();
    // dbg!(numbers);    
    // dbg!(lines);  

    for number in numbers{
        
        for board in &mut boards{
            board.mark(number);
            if board.check(){
                println!("{:#?}", number * board.sum());
                return ;

            }
        }
    }
}