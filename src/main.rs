use std::io;
use std::process::exit;

fn main() {
    println!("Enter a password:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    if input.len() >32 {
        println!("Too long");
        exit(1);
    }
    else if input.len()<8 {
        println!("Too short");
        exit(1);
    }
    let key = generate_key(input.trim());
    println!("{:?}",key);
    let subkey_1 = generate_subkey(key[0..4].to_vec());
    let subkey_2 = generate_subkey(key[4..8].to_vec());
    let subkey_3 = generate_subkey(key[8..12].to_vec());
    let subkey_4 = generate_subkey(key[12..16].to_vec());
    let subkey_5 = generate_subkey(key[16..20].to_vec());
    let subkey_6 = generate_subkey(key[20..24].to_vec());
    let subkey_7 = generate_subkey(key[24..28].to_vec());
    let subkey_8 = generate_subkey(key[28..32].to_vec());

    print_hex(key);
    print_hex(subkey_1);
    print_hex(subkey_2);
    print_hex(subkey_3);
    print_hex(subkey_4);
    print_hex(subkey_5);
    print_hex(subkey_6);
    print_hex(subkey_7);
    print_hex(subkey_8);
}

fn generate_key(str:&str) -> Vec<u8> {
    let mut bytes = str.as_bytes().to_vec();
    let mut number:Vec<u8> = Vec::new();
    let mut number2:Vec<u8> = Vec::new();
    let mut index= 0;
    while bytes.len() < 32 {
            let last_byte: i16 = 2048+bytes[bytes.len()-1] as i16/4 - 8*bytes[index+1] as i16;
            index+=1;
            let new_byte: u8 = 128u8.wrapping_sub(last_byte as u8);
            bytes.push(new_byte);
    }
    for byte in bytes {
        if byte %2 == 0{
            number.push(byte);
        }
        else{
            number2.push(byte);
        }
    }
    [number,number2].concat()
}
fn generate_subkey(vector:Vec<u8>) -> Vec<u8> {
    let mut bytes = vector.clone();
    let mut number:Vec<u8> = Vec::new();
    let mut number2:Vec<u8> = Vec::new();
    let mut index= 0;
    while bytes.len() < 12 {
        let last_byte: i16 = 2048+bytes[bytes.len()-1] as i16/4 - 8*bytes[index+1] as i16;
        index+=1;
        let new_byte: u8 = 128u8.wrapping_sub(last_byte as u8);
        bytes.push(new_byte);}
    for byte in bytes {
        if byte %2 == 0{
            number.push(byte);}
        else{
            number2.push(byte);}}
    [number,number2].concat()
}
fn print_hex(vec:Vec<u8>) {
    let hex_string = vec.iter()
        .map(|byte| format!("{:X}", byte))
        .collect::<Vec<String>>()
        .join("-");
    println!("{}", hex_string);
}