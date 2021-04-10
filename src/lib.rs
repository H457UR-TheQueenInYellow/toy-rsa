extern crate rand;

///modexp takes three u64 values, x, y, and m, then recursively determines the resulting exponentiation of x and y, mod m. Errors if m is zero
pub fn modexp(x: u64, y: u64, m: u64) -> u64 {
    if m == 0 {
        error("modexp: error: m cannot be zero");
    } else if x == 0 {
        return 0;
    } else if y == 0 {
        return 1;
    }

    let mut z: u64 = modexp(x, y / 2, m);

    z = (z * z) % m;
    if y % 2 == 1 {
        z = (z * x) % m;
    }
    z
}

///Upon finding an error, this function is called to display a (provided) custom message to the user, then cleanly exit the program. During tests, it instead panics following the message.
pub fn error(msg: &str) -> ! {
    eprintln!("{:?}", msg);
    #[cfg(test)]
    panic!("error");
    #[cfg(not(test))]
    std::process::exit(1);
}

///Primegen generates random integers between 2^30 and 2^32 (this is a ***toy*** implementation) until it generates one that the primality function confirms to be prime.
pub fn primegen() -> u64 {
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
pub fn primality(num: u64) -> bool {
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
pub fn gcd(x: u64, mut y: u64) -> u64 {
    while y != 0 {
        if y < x {
            return gcd(y, x);
        }
        y %= x;
    }
    x
}

///Finds least common multiple of the given x and y; utilizes gcd function
pub fn lcm(x: u64, y: u64) -> u64 {
    (x * y) / gcd(x, y)
}

///This function finds the modular inverse, if one exists, of two input u64s; presented to you through the power of ~~deep mathematical understanding~~ borrowing code from the C# section of www.geeksforgeeks.org/multiplicative-inverse-under-modulo-m/
pub fn modinv(a: u64, m: u64) -> u64 {
    let mut x: u64 = 1;

    while x < m {
        if ((a % m) * (x % m)) % m == 1 {
            return x;
        }
        x += 1;
    }
    1
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
