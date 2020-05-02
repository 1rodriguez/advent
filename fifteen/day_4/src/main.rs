fn main() {
    const _KEY : &str = "iwrupvqb";
    const _H : (u32, u32, u32, u32) = (0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476);

    test();
}

fn into_bits(input : &str) -> Vec<u32> {
    // returns vector containing binary representation of input string
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
    
//    while bits.len() < 8 {
//        bits.insert(0,0); // padding until output is a byte long
//    }

    bits // bit vector follows little-endian convention (least significant bit = 1)
}

fn append_bits(bit_stream : Vec<u32>) -> Vec<u32> {
    // returns padded bit_stream
    // first a 1 is added to the end of the stream, followed by 0s
    // the stream is padded until its length is 448 mod 512
    // a bit_stream with length 448 is also padded (add 1, then 511 0s)

    let lng = bit_stream.len();
    let mod_length = lng % 512;
    let pad; // stores zero pad-length
    let mut pad_stream = bit_stream;

    pad = match mod_length {
        1 ..= 447 => 447 - mod_length,
        449 ..= 511 => 959 - mod_length,
        _ => 511,
    };
    
    let mut pad_vec : Vec<u32> = vec![0; pad]; // generating 0 vector according to original length
    pad_stream.push(1); // 1 always appended before 0s
    pad_stream.append(&mut pad_vec);

    pad_stream
    
}

fn to_64_bit(bit_stream : Vec<u32>) -> Vec<u32> {
    // returns 64-bit version of bit stream (mod 2^64)

   
}

fn test() {
    // bit stream of a = 97 in binary (ASCII representation of "a")
    assert_eq!(into_bits("a"), [1, 1, 0, 0 , 0, 0, 1]); 
    
    assert_eq!(448, append_bits(vec![1;449]).len() % 512);
    assert_eq!(448, append_bits(vec![1;447]).len() % 512);
    assert_eq!(448, append_bits(vec![1;448]).len() % 512);
    assert_eq!(448, append_bits(vec![1;969]).len() % 512);

    assert_eq!(448, append_bits(into_bits("a")).len() % 512);
}
