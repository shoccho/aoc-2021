struct LanternFish{
    days:usize,
    mature: bool
}
impl LanternFish{
    fn new()->Self{
        LanternFish{
            days:8,
            mature:false
        }
    }
    fn age(&mut self)->Option<Self>{
        if self.days>0{    
            self.days-=1;
            return None;
        }else{
            self.days = 6;
            if !self.mature {
                self.mature = true;
            }
            return Some(LanternFish::new());
        }
    }
}
fn main() {
    let start = include_str!("input").split(',').map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    simulate(&start, 26);
    cheat(&start);

}

fn simulate(start:&[usize],days:usize){
    let mut school :Vec<LanternFish> = Vec::new();
    for day in start{
        school.push(LanternFish{
            days:*day,
            mature:true
        })
    }
    for _ in 0..days{
        let mut childs :Vec<LanternFish> = Vec::new();
        for fish in &mut school{
            if let Some(new_fish) = fish.age() { childs.push(new_fish) }
        }
        school.append(&mut childs);
    }
    
    println!("school size after {} days is {}",days,school.len());

}

fn cheat(start:&[usize]){

    let mut days:[u128;270] = [0;270];
    
    for fish in start{
        days[*fish]+=1;
    }
    for d in 0..256{
        days[d+7]+=days[d];
        days[d+9]+=days[d];
    }
    print!("part 2 {}\n",days.iter().take(256).sum::<u128>()+start.len() as u128);

}