use std::process::exit;

mod retcrypt_module_prototype;

fn main() {
    if let Err(e) = retcrypt_module_prototype::check_config_file() {
        println!("Error creating config file: {}", e);
    }
    match retcrypt_module_prototype::read_config() {
        Ok(config) => {
            let input = retcrypt_module_prototype::get_input("Enter a password: ");
            if input.len() >64 {
                println!("Too long");
                exit(1);
            }
            else if input.len()<8 {
                println!("Too short");
                exit(1);
            }
            let channel:Vec<u8> = retcrypt_module_prototype::decode_base64(config.channel.channel_data);
            let subkeys = retcrypt_module_prototype::generate_subkeys(retcrypt_module_prototype::expand_key(&input.trim(),config.key.bit_size),config.key.subkey_number);
            let data = retcrypt_module_prototype::get_input("Enter a text to encrypt:");
            let input = data.trim().as_bytes().to_vec();
            println!("Input: {:?}",input);
            retcrypt_module_prototype::print_hex(input);
            let encrypted_data = retcrypt_module_prototype::start_encrypt(data.trim().as_bytes().to_vec(), subkeys.clone(),config.key.subkey_bit_size, channel.clone());
            println!("Encrypted Data: {:?}",encrypted_data);
            retcrypt_module_prototype::print_hex(encrypted_data.clone());
            let decrypted_data =retcrypt_module_prototype::start_decrypt(retcrypt_module_prototype::encode_base64(encrypted_data.clone()), subkeys,config.key.subkey_bit_size,  channel);
            println!("Decrypted Data: {:?}",decrypted_data);
            retcrypt_module_prototype::print_hex(decrypted_data.clone());
            for x in decrypted_data {
                print!("{}",x as char);
            }
        }
        Err(err) => {
            println!("Error reading config: {}", err);
        }
    }

}