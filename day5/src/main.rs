use std::collections::HashMap;
use std::cmp;

#[derive(Debug,PartialEq,Eq,Hash,Clone)]

struct Point {
    x: i32,
    y:i32
}
// impl PartialEq for Point{
//     fn eq(&self, other:&Self)->bool{
//         self.x==other.x && self.y==other.y
//     }
// }

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Point{

    fn new(x:i32,y:i32)->Self{
        Point{
            x,
            y
        }
    }
    fn from_str(line: &str)->Self{
        let mut tokens = line.trim().split(',');
        Point{
            x:tokens.next().unwrap().parse().unwrap(),
            y:tokens.next().unwrap().parse().unwrap(),
                        
        }
    }

}
impl Line {
    // add code here
    fn parse(line: &str)->Self{
        let mut tokens =
            line.split("->");
        Line{
            start : Point::from_str(tokens.next().unwrap()),
            end:Point::from_str(tokens.next().unwrap())
        }
    }
    fn straight(&self)->bool{
        self.start.x==self.end.x || self.start.y==self.end.y
    }
    fn get_points(&self)->Vec<Point>{      
        let mut ret :Vec<Point> = Vec::new();
        
        let x1= cmp::min(self.start.x,self.end.x);
        let x2= cmp::max(self.start.x,self.end.x);
        let mut dx=1;
        let mut dy=1;

        if self.start.x>self.end.x {
            dx=-1;
        }
        if self.start.y >self.end.y{
           dy=-1
        }
        let diff  =x2-x1;
        if self.straight() {
            return self.get_straight_points();
        } else{

            for i in 0..=diff{

                ret.push(Point::new(self.start.x+(dx*i),self.start.y+(dy*i)));
            }
        }
        ret
    }   
    
    fn get_straight_points(&self)->Vec<Point>{
        

        let m;
        let n;
        let fixed;
        let mut ret :Vec<Point> = Vec::new();
        
        if self.start.x==self.end.x{
           fixed=self.start.x;
            m= cmp::min(self.start.y,self.end.y);
            n= cmp::max(self.start.y,self.end.y);
            for i in m..=n{
                ret.push(Point::new(fixed,i));
            }
            

        }else{
           fixed=self.start.y;
           
            m= cmp::min(self.start.x,self.end.x);
            n= cmp::max(self.start.x,self.end.x);
            for i in m..=n{
                ret.push(Point::new(i,fixed));
            }
        }

        ret
    }
}
fn main() {
    let lines = include_str!("input").lines().map(|x| Line::parse(x)).collect::<Vec<Line>>();
    // dbg!(input);
    solve1(&lines);
    solve2(&lines);
}

fn solve1(lines: &Vec<Line>){
    let mut map :HashMap<Point,bool>=HashMap::new();
    let mut res =0;
    for line in lines{
        if line.straight(){
            // println!("{:?}",line );
            let points =line.get_points();
            for point in points{
                // println!("{:?}",point );
                match map.get(&point){

                    Some(false)=>{res+=1;*map.get_mut(&point).unwrap()=true;},
                    Some(true)=>{},
                    None=> {map.insert(point,false);},
                }
            }
        }

    }
    println!("{:?}",res );
}
fn solve2(lines: &Vec<Line>){
    let mut map :HashMap<Point,bool>=HashMap::new();
    let mut res =0;
    for line in lines{
       
        // println!("{:?}",line );
        let points =line.get_points();
        // println!("for line{:#?}",line );
        for point in points{
            // println!("{:?}",point );
            match map.get(&point){

                Some(false)=>{res+=1; *map.get_mut(&point).unwrap()=true;},
                Some(true)=>{},
                None=> {map.insert(point,false);},
            }
        }
    

    }
    println!("part 2 {:?}",res );
}