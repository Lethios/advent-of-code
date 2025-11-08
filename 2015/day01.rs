// https://adventofcode.com/2015/day/1

fn main() {
    let input: &str = "<INPUT>";
    let mut floor: i32 = 0;
    
    for chr in input.chars() {
        match chr {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
    }
    
    println!("Floor {floor}");
}
