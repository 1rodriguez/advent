fn main() {
    const _KEY : &str = "iwrupvqb";
    const _H : (u32, u32, u32, u32) = (0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476);

    //assert_eq!([0],_md4("hello"));
    let vec = Vec::new(0; 5);
    assert_eq!(vec, [0, 0, 0, 0, 0]);
}

fn _md4(input : &str) -> Vec<u32> {
    // returns hex bytecode value of encrypted string
    //
    // left rotation: bits that fall off at left added at right
    
    let bytes = input.to_string().into_bytes();
    let mut bits : Vec<u32> = Vec::new();

    for entry in bytes {
        let bin : String =  format!("{:b}", entry);
        for bit in bin.chars() {
            bits.push(bit.to_digit(10).expect("could not read bit"));
        }
    }
    bits
}
