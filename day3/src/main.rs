fn process_bits(idx: usize, data: &Vec<&str>) -> (i32,i32) {
        
    let mut zero = 0;
    let mut one = 0;

    for bit in data {
        if bit.as_bytes()[idx] == 48 {
            zero += 1;
        } else {
            one += 1;
        }
    }
    if zero>one{
        (0,1)
    }else {
        (1,0)
    }
}


fn main(){
    
    let bit_stream = include_str!("input").lines().collect::<Vec<&str>>();  
    let length = bit_stream.first().unwrap().len();

    solve1(&bit_stream,length);
    solve2(&bit_stream,length);
}

fn solve1(bits :&Vec<&str>,length:usize){
    let mut gamma = 0;
    let mut epsilon = 0;
    for idx in 0..length{
        let (g,e) = process_bits(idx,&bits);
        gamma <<= 1;
        gamma |= g;
        epsilon <<= 1;
        epsilon |= e;
    }
    println!("{:?}",gamma * epsilon );
}

fn solve2(bits:&Vec<&str>, length:usize){
    let mut o2 = bits.clone();
    let mut co2 = bits.clone();
    for idx in 0..length{

        let (g,_e) = process_bits(idx,&o2);
       
        if o2.len()!=1 {
            o2 = o2.into_iter().filter(|b| b.as_bytes()[idx] == 48+g as u8).collect();    
        }
        let (_g,e) = process_bits(idx,&co2);
        if co2.len()!=1 {
            co2 = co2.into_iter().filter(|b| b.as_bytes()[idx] == 48+e as u8).collect();    
        }
        
    }
    println!("{}",i32::from_str_radix(o2.first().unwrap(),2).unwrap()*i32::from_str_radix(co2.first().unwrap(),2).unwrap())
}