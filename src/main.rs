use toyrsa::*;

fn main() {
    let encryptedinfo: (u64, u64, u64) = encrypt(12345);

    println!("Your encrypted message is: {}", encryptedinfo.2);

    println!("Your decrypted message probably isn't 12345, as was put in. This is because I don't really understand modular inverses. \nOh well. Here's what we've got for you anyway: {}", decrypt(encryptedinfo.0,encryptedinfo.1,encryptedinfo.2));
}
