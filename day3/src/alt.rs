#[derive(Debug)]
enum Bit {
	high,
	low,
}

fn process_bits(idx: usize, data: &Vec<&str>) -> (u8,u8) {
        
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
    	return (0,1);
    }else {
    	return (1,0);
    }
}

fn main(){

	let input = include_str!("input");
	let length = input.first().chars().len();
	let bit_stream = input.lines().collect::<Vec<&str>>();	
	
}
fn solve1(bits :Vec<&str>,length:usize){
	let mut gamma = 0;
	let mut epsilon =0;
	for idx in 0..length{
		let (g,e) = process_bits(idx,bits);
		gamma <<= 1;
		gamma |= g;
		epsilion <<= 1;
		epsilion |= e;
	}
	println!("{:?}",gamma*epsilion);
}