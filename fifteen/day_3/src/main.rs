use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    // part 2: santa reads even-indexed instructions; robo-santa reads odd instructions
    let mut s_x : i32 = 0;
    let mut s_y : i32 = 0;
    let mut r_x : i32 = 0;
    let mut r_y : i32 = 0;

    let contents = read_to_string("directions.txt")
        .expect("error in reading file");

    let mut houses : HashMap<String, u32> = HashMap::new();

    houses.insert("0,0".to_string(), 2); // two presents at starting house

    for (i, c) in contents.chars().enumerate() {
        match (c, i % 2) {
            ('^', 0)  => s_y += 1,
            ('v', 0) => s_y -= 1,
            ('<', 0) => s_x -= 1,
            ('>',0) => s_x += 1, 
            ('^', _) => r_y += 1, // ignoring modular index as previous arms have already been checked
            ('v', _) => r_y -= 1,
            ('<', _) => r_x -= 1,
            ('>', _) => r_x += 1,
            _ => continue,
        };

        if i % 2 == 0 {
            map_parser(&mut houses, s_x, s_y); // even index instruction: santa delivers
        } else {
            map_parser(&mut houses, r_x, r_y); // odd index: robo-santa delivers
        }
    }

    print!("{} houses visisted", houses.len());

}

fn _santa_route() {
    let mut x : i32 = 0;
    let mut y : i32 = 0;

    let contents = read_to_string("directions.txt")
        .expect("error in reading file");

    let mut houses : HashMap<String, u32>= HashMap::new();

    houses.insert("0,0".to_string(), 1); // present at initial location

    for c in contents.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            '>' => x += 1, 
            _ => continue,
        };

        map_parser(&mut houses, x, y);

    }

    print!("{} houses were visited", houses.len());
}

fn map_parser(map : &mut HashMap<String, u32>, x_coord : i32, y_coord : i32) { // mut ref to avoid issue with 'houses' HashMap moving
    // handling the HashMap containing the house addresses and times visited
    let x_coord = x_coord.to_string();
    let y_coord = y_coord.to_string();

    // translating coordinates to x,y key format for comparison
    let mut address = String::new();
    address.push_str(&x_coord);
    address.push_str(",");
    address.push_str(&y_coord);

    if let Some(x) = map.get_mut(&address) {
       *x += 1; 
    } else {
        map.insert(address, 1);
    }

    }

