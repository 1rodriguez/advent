use std::fs::read_to_string;
use std::cmp::min;

fn main() {
    let mut total : u32 = 0;
    let contents = read_to_string("wrapping.txt").expect("error in reading file");
    
    let contents : Vec<&str> = contents.split("\n").collect();

    for entry in contents {
        if entry == "" { break; } // empty string when attempting to parse end of file
        total += process_entry(entry.to_string());
    }
    print!("The total length of ribbon required is {}", total);
}

fn process_entry(line : String) -> u32 { 
    let line : Vec<&str> = line.split('x').collect();

    let length : u32 = line[0]
        .parse()
        .expect("could not parse length");
    let width : u32 = line[1]
        .parse()
        .expect("could not parse width");
    let height : u32 = line[2]
        .parse()
        .expect("could not parse height");

    let side_a : u32 = length * width;
    let side_b : u32 = length * height;
    let side_c : u32 = width * height;

    let _slack : u32 = min(side_a, min(side_b, side_c)); // slack defined as area of smallest side

    let _area : u32 = 2 * (side_a + side_b + side_c);
    // slack + area if finding surface area of wrapping paper
    ribbon(length, width, height)
}

fn ribbon(length : u32, width : u32, height : u32) -> u32 {
    let bow : u32 = length * width * height;
    let surface : u32 = two_min(length, width, height);

    bow + surface
}

fn two_min(length: u32, width : u32, height : u32) -> u32 {
    // returns sum of smallest two numbers
    let lowest : u32 = min(length, min(width, height));
    let mid : u32;
    
    if length == lowest {
        mid = min(width, height);
    } else if width == lowest {
        mid = min(length, height);
    } else {
        mid = min(length, width);
    } // Why does a match statement not work? lower two arms unreachable
    // mid = match lowest {
    // length => min(...),
    // width => min(...),
    // height => min(...),
    // }

    2 * (lowest + mid)
}
