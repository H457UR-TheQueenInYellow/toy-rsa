use toyrsa::*;

fn main() {
    let msg = 12345;
    let encryptedinfo: (u64, u64, u64) = encrypt(msg);

    println!("Your encrypted message is: {}", encryptedinfo.2);

    println!("You encrypted the following: {}\nThe output from decryption is: {}", msg, decrypt(encryptedinfo.0,encryptedinfo.1,encryptedinfo.2));
}
