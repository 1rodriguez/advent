use std::fs::read_to_string;

fn main() {
    let mut basement: bool = false;
    let mut count = 0;
    let contents = read_to_string("floors.txt").expect("error in reading file");

    for (i, c) in contents.chars().enumerate() {
        if count == -1 && !basement {
            let index = i;
            basement = true;

            print!("Santa has entered the basement at: {} \n", index);
        }
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => continue,
        }
    }
    println!("Santa must go to floor: {}", count);
}
