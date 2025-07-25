// Auralis takımının güvenli uydu yarışması için geliştirilmiş prototiptir.
// This is a prototype developed for the Auralis team's secure satellite competition.

use rand::Rng;
use base64::{Engine as _, engine::{general_purpose}};
use std::fs::File;
use std::path::Path;
use std::io::{self, Write};
use serde_derive::Serialize;
use serde_derive::Deserialize;
use toml;
use std::fs;


#[derive(Serialize, Deserialize)]
pub struct Config {
    pub(crate) channel: Channel,
    pub(crate) key: Keys,
}
#[derive(Serialize, Deserialize)]
pub struct Keys {
    pub(crate) bit_size: usize,
    pub(crate) subkey_number:usize,
    pub(crate) subkey_bit_size:usize,
}#[derive(Serialize, Deserialize)]
pub struct Channel {
    pub(crate) channel_data: String,
}

pub fn check_config_file() -> io::Result<()> {
    let filepath = "retcrypt-config.toml";
    if Path::new(filepath).exists() {
        return Ok(());
    }
    let config = Config {
        key: Keys {
            bit_size:512,
            subkey_number:16,
            subkey_bit_size:128,
        },
        channel: Channel {
            channel_data: encode_base64(generate_channel()),
        },
    };
    let toml_str = toml::to_string(&config).expect("Failed to serialize config");
    let mut file = File::create(filepath)?;
    file.write_all(toml_str.as_bytes())?;
    println!("Configuration file created successfully!");
    Ok(())
}
pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("retcrypt-config.toml")?;
    let config: Config = toml::de::from_str(&config_str)?;
    Ok(config)
}
pub fn get_input(str:&str) -> String {
    println!("{}",str);
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Failed to read line");
    data.trim().to_string()
}
pub fn generate_channel() ->Vec<u8>{
    let mut channel = Vec::new();
    while channel.len() < 64{
        let mut rng = rand::rng();
        let number:u8 =rng.random_range(1..=255) as u8;
        channel.push(number);
    }
    channel
}
pub fn generate_subkeys(key: Vec<u8>,count:usize) -> Vec<Vec<u8>> {
    let mut subkeys = Vec::new();
    for i in 0..count {
        subkeys.push(key[(i * 4)..(i * 4 + 4)].to_vec());
    }
    subkeys
}
pub fn expand_key(str:&str,ksize:usize) -> Vec<u8> {
    let mut bytes = str.as_bytes().to_vec();
    let mut number:Vec<u8> = Vec::new();
    let mut number2:Vec<u8> = Vec::new();
    let mut index= 0;
    while bytes.len() < ksize/8 {
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
pub fn expand_subkey(vector:Vec<u8>,size:usize) -> Vec<u8> {
    let mut bytes = vector.clone();
    let mut number:Vec<u8> = Vec::new();
    let mut number2:Vec<u8> = Vec::new();
    let mut index= 0;
    while bytes.len() < size/8 {
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
pub fn encrypt_data(data:Vec<u8>,key:Vec<u8>,ch:Vec<u8>)->Vec<u8>{
    let mut encrypted_data:Vec<u8> = Vec::new();
    let key_length = key.len();
    let channel_length = ch.len();
    for byte in data.iter() {
        let mut crypted_byte = *byte;
        for x in 0..channel_length{
            crypted_byte = !(crypted_byte ^ ch[x]);
        }
        for y in 0..key_length{
            crypted_byte = crypted_byte ^ key[y];
        }
        encrypted_data.push(crypted_byte);
    }
    encrypted_data
}
pub fn decrypt_data(data:Vec<u8>,key:Vec<u8>,channel: Vec<u8>)->Vec<u8>{
    let mut decrypted_data:Vec<u8> = Vec::new();
    let key_length = key.len();
    let channel_length = channel.len();
    for byte in data.iter() {
        let mut crypted_byte = *byte;
        for y in 0..key_length{
            crypted_byte = crypted_byte ^ key[key_length-y-1];
        }
        for x in 0..channel_length{
            crypted_byte = !(crypted_byte ^ channel[x]);
        }
        decrypted_data.push(crypted_byte);
    }
    decrypted_data
}
pub fn print_hex(vec:Vec<u8>) {
    let hex_string = vec.iter()
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<String>>()
        .join("");
    println!("{:?}", hex_string);
}
pub fn encode_base64(vec:Vec<u8>)-> String {
    general_purpose::STANDARD.encode(vec.clone()).trim().to_string()
}
pub fn decode_base64(str:String) -> Vec<u8> {
    general_purpose::STANDARD.decode(str.trim().to_string()).unwrap()
}
pub fn start_encrypt(data: Vec<u8>, subkeys: Vec<Vec<u8>>, size:usize,channel: Vec<u8>) -> Vec<u8> {
    let mut encrypted_data = data;
    for subkey in subkeys {
        encrypted_data = encrypt_data(encrypted_data, expand_subkey(subkey,size), channel.clone());
    }
    encrypted_data
}
pub fn start_decrypt(data:String, subkeys: Vec<Vec<u8>>, size:usize,channel: Vec<u8>) -> Vec<u8> {
    let mut decrypted_data = decode_base64(data.trim().to_string());
    for subkey in subkeys {
        decrypted_data = decrypt_data(decrypted_data,expand_subkey(subkey,size),channel.clone());
    }
    decrypted_data
}