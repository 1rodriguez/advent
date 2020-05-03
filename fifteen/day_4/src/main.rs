fn main() {
    const _KEY : &str = "iwrupvqb";
    const _H : (u32, u32, u32, u32) = (0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476);

    test();
}

fn into_le_bits(input : &str) -> Vec<u32> {
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
    
    bits.reverse();
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

fn to_64_bit(input : &str) -> Vec<u32> { 
    // returns 64-bit version of bit stream (mod 2^64)

    let max : u128 = 2_u128.pow(64);

    let bytes = input.to_string().into_bytes();
    let mut bin_string : String = "".to_string();

    for entry in bytes {
        let bin : String = format!("{:b}", entry);
        bin_string.push_str(&bin);
    }

    let bin_num : u128 = u128::from_str_radix(&bin_string, 2).unwrap();
    let bin_num : u128 = bin_num % max;

    let bytes : [u8;16] = bin_num.to_be_bytes();
    let mut bits : Vec<u32> = Vec::new();
   
    for entry in bytes.iter() {
        let bin_string : String = format!("{:b}", entry);
        for n in bin_string.chars() {
            bits.push(n.to_digit(10).expect("could not read bit"));
        }
    }
    bits.reverse();

    while bits.len() < 64 {
        bits.push(0);
    }

    bits
}

fn preprocess(input : &str) -> Vec<u32> {
    // returns a vector containing preprocessed string (in binary)
    // output vector length will always be a multiple of 512
    let mut input_le = into_le_bits(input);
    let mut input_64 = to_64_bit(input);

    input_le.append(&mut input_64);

    let processed = input_le;
    
    processed
}

fn test() {
    // bit stream of a = 97 in binary (ASCII representation of "a")
    let mut a_le : Vec<u32> = [1, 0, 0, 0, 0, 1, 1].to_vec(); // a in little-endian (le) form
    assert_eq!(into_le_bits("a"), a_le); 
    
    assert_eq!(448, append_bits(vec![1;449]).len() % 512);
    assert_eq!(448, append_bits(vec![1;447]).len() % 512);
    assert_eq!(448, append_bits(vec![1;448]).len() % 512);
    assert_eq!(448, append_bits(vec![1;969]).len() % 512);

    assert_eq!(448, append_bits(into_le_bits("a")).len() % 512);

    a_le.append(&mut vec![0; 57]);
    let a_le_64 : Vec<u32> = a_le;

    assert_eq!(a_le_64, to_64_bit("a"));

}
