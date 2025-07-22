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
    let key = generate_key(&input.trim());
    let subkey_1 = generate_subkey(key[0..4].to_vec());
    let subkey_2 = generate_subkey(key[4..8].to_vec());
    let subkey_3 = generate_subkey(key[8..12].to_vec());
    let subkey_4 = generate_subkey(key[12..16].to_vec());
    let subkey_5 = generate_subkey(key[16..20].to_vec());
    let subkey_6 = generate_subkey(key[20..24].to_vec());
    let subkey_7 = generate_subkey(key[24..28].to_vec());
    let subkey_8 = generate_subkey(key[28..32].to_vec());

    println!("Enter a text to encrypt:");
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Failed to read line");
    //print_hex(data.as_bytes().to_vec());

    let encrytped_data = encrypt_data(data.trim().as_bytes().to_vec(),subkey_1.clone());
    let encrytped2_data = encrypt_data(encrytped_data.clone(),subkey_2.clone());
    let encrytped3_data = encrypt_data(encrytped2_data,subkey_3.clone());
    let encrytped4_data = encrypt_data(encrytped3_data,subkey_4.clone());
    let encrytped5_data = encrypt_data(encrytped4_data,subkey_5.clone());
    let encrytped6_data = encrypt_data(encrytped5_data,subkey_6.clone());
    let encrytped7_data = encrypt_data(encrytped6_data,subkey_7.clone());
    let encrytped8_data = encrypt_data(encrytped7_data,subkey_8.clone());
    //decrypt_data(encrytped_data.clone(),subkey_1.clone());
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
            let last_byte: i16 = bytes[bytes.len()-1] as i16 - bytes[index+1] as i16;
            index+=1;
            let new_byte: u8 = 128u8.wrapping_sub(last_byte as u8);
            bytes.push(new_byte);
    }
    for byte in &bytes {
        if byte %2 == 0{
            number.push(*byte);
        }
        else{
            number2.push(*byte);
        }
    }
    bytes
}
fn generate_subkey(vector:Vec<u8>) -> Vec<u8> {
    let mut bytes = vector.clone();
    let mut number:Vec<u8> = Vec::new();
    let mut number2:Vec<u8> = Vec::new();
    let mut index= 0;
    while bytes.len() < 12 {
        let last_byte: i16 = bytes[bytes.len()-1] as i16 - bytes[index+1] as i16;
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
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<String>>()
        .join("");
    println!("{:?}", vec.clone());
}

fn encrypt_data(data:Vec<u8>,key:Vec<u8>)->Vec<u8>{
    //println!("{:?}, {:?}",data,data.len());
    let mut encrypted_data:Vec<u8> = Vec::new();
    let mut byte32data :Vec<u8> = Vec::new();
    for bytes in data.clone(){
        while byte32data.len() < 4 {
            byte32data.push(bytes);
        }
        if byte32data.len() == 4{
            
        }
    }
    for byte in data.clone() {
        let mut index = 1;
        let mut crypted_byte = !(byte ^ key[0]);
        while index < key.len(){
            crypted_byte = crypted_byte ^ key[index];
            index = index+1;
        }
        encrypted_data.push(crypted_byte);
    }
    println!("{:?} , {:?}",encrypted_data,encrypted_data.len());
    //print_hex(encrypted_data);
    encrypted_data
}

fn decrypt_data(data:Vec<u8>,key:Vec<u8>){
    println!("{:?}, {:?}",data,data.len());
    let mut decrypted_data:Vec<u8> = Vec::new();
    for byte in data {
        let mut index = 1;
        let mut crypted_byte = byte;
        while index < key.len(){
            crypted_byte = crypted_byte ^ key[key.len()-index];
            index=index+1;
        }
        crypted_byte = !(crypted_byte^key[0]);
        decrypted_data.push(crypted_byte);
    }
    println!("{:?} , {:?}",decrypted_data,decrypted_data.len());
    for char in decrypted_data {
        print!("{}",char as char);
    }
}