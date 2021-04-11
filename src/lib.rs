extern crate rand;
use std::convert::TryFrom;

///modexp takes three u64 values, x, y, and m, then recursively determines the resulting exponentiation of x and y, mod m. Errors if m is zero
fn modexp(x: u64, y: u64, m: u64) -> u64 {
    if m == 0 {
        error("modexp: error: m cannot be zero");
    } else if x == 0 {
        return 0;
    } else if y == 0 {
        return 1;
    }

    let mut z: u128 = u128::from(modexp(x, y / 2, m));

    z = (z * z) % u128::from(m);
    if y % 2 == 1 {
        z = (z * u128::from(x)) % u128::from(m);
    }

    u64::try_from(z).unwrap()
}

///Upon finding an error, this function is called to display a (provided) custom message to the user, then cleanly exit the program. During tests, it instead panics following the message.
fn error(msg: &str) -> ! {
    eprintln!("{:?}", msg);
    #[cfg(test)]
    panic!("error");
    #[cfg(not(test))]
    std::process::exit(1);
}

///Primegen generates random integers between 2^30 and 2^32 (this is a ***toy*** implementation) until it generates one that the primality function confirms to be prime.
fn primegen() -> u64 {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let base: u64 = 2;
    let mut num: u64 = 4;

    while !primality(num) {
        num = rng.gen_range(base.pow(30), base.pow(31));
    }

    num
}

///This is the rust implementation of the C# code found at https://en.wikipedia.org/wiki/Primality_test
fn primality(num: u64) -> bool {
    if num == 2 || num == 3 {
        return true;
    } else if num <= 1 || num % 2 == 0 || num % 3 == 0 {
        return false;
    }

    let mut i: u64 = 5;

    while i.pow(2) <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

///Finds the greatest common denominator of x and y
fn gcd(x: u64, mut y: u64) -> u64 {
    while y != 0 {
        if y < x {
            return gcd(y, x);
        }
        y %= x;
    }
    x
}

///Finds least common multiple of the given x and y; utilizes gcd function
fn lcm(x: u64, y: u64) -> u64 {
    (x * y) / gcd(x, y)
}

///This function finds the modular inverse, if one exists, of two input u64s; presented to you through the power of ~~deep mathematical understanding~~ borrowing code from the C# section of www.geeksforgeeks.org/multiplicative-inverse-under-modulo-m/
/*pub fn modinv(a: u64, m: u64) -> u64 {
    let mut x: u64 = 1;

    while x < m {
        if ((a % m) * (x % m)) % m == 1 {
            return x;
        }
        x += 1;
    }
    1
}*/

///This is the modular multiplicative inverse function. I did not write this. My grasp of advanced maths is not (currently) that strong; rather, I borrowed it from Professor Bart Massey's toy-rsa library (library found here: https://github.com/pdx-cs-rust/toy-rsa-lib/blob/master/src/lib.rs, rustdoc found here: https://pdx-cs-rust.github.io/toy-rsa-lib/toy_rsa_lib/index.html)
#[allow(clippy::many_single_char_names)]
pub fn modinv(a: u64, m: u64) -> u64 {
    let mut a = a as i128;
    let mut m = m as i128;
    let m0 = m;
    let mut y = 0;
    let mut x = 1;
    if m == 1 {
        return 0;
    }

    while a > 1 {
        // q is quotient.
        let q = a / m;
        let mut t = m;

        // m is remainder now; process same as
        // Euclid's Algorithm.
        m = a % m;
        a = t;
        t = y;

        // Update y and x.
        y = x - q * y;
        x = t;
    }

    // Make x positive.
    if x < 0 {
        x += m0;
    }

    // XXX This conversion should never fail, as `x` should
    // always be positive and less than `m` at this point.
    u64::try_from(x).unwrap()
}

pub fn genkey() -> (u64, u64) {
    let p: u64 = primegen();
    let mut q: u64 = primegen();
    let e: u64 = 65537;

    while e >= lcm(p - 1, q - 1) && gcd(e, lcm(p - 1, q - 1)) != 1 {
        q = primegen();
    }
    (p, q)
}

pub fn encrypt(msg: u64) -> (u64,u64,u64) {
    let key: (u64, u64) = genkey();
    let pubkey: u64 = key.0 * key.1;
    println!("Your private key is: p = {}, q = {}\nDo not share or lose these; they're vital to decrypting your message\n\nYour public key is: {}",key.0,key.1,pubkey);
    let encryptmsg: u64 = modexp(msg, 65537, pubkey);
    //println!("Your encrypted message is: {}", encryptmsg);
    (key.0,key.1,encryptmsg)
}

pub fn decrypt(p: u64, q: u64, msg: u64) -> u64 {
    let d = modinv(65537,lcm(p-1,q-1));
    let dcrpt: u128 = msg.u64::pow(d) % (p*q);
    dcrpt
}

///Series of tests that check the modexp function
#[cfg(test)]
mod modexp_tests {
    use super::*;

    ///Hands the modexp function a series of known-good inputs, and checks the output value
    #[test]
    fn goodexp() {
        assert!(modexp(2, 20, 17) == 16);
        assert!(modexp(4294967295, 4294967295, 7) == 6);
        assert!(modexp(0, 0, 1) == 0);
        assert!(modexp(1, 0, 1) == 1);
        assert!(modexp(1, 1, 1) == 0);
        assert!(modexp(1, 1, 2) == 1);
    }

    ///Tries to give modexp an m with a zero value; should panic.
    #[test]
    #[should_panic]
    fn badexp() {
        modexp(2, 20, 0);
    }
}

#[test]
fn test_modinverse() {
    const BIGM: u64 = u64::max_value() - 58;
    let m0 = 0xffff_ffff_ffff_f000;
    let mi = modinv(m0, BIGM);
    let m = modinv(mi, BIGM);
    assert_eq!(m0, m);
}