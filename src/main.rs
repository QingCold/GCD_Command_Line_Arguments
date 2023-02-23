use std::{env, str::FromStr};

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t: u64 = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(4, 6), 2);
    assert_eq!(gcd(24, 18), 6);
    assert_eq!(gcd(55, 44), 11);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
        3 * 7 * 11 * 13 * 19),
        3 * 11);
}

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing arguments"));
    }

    if numbers.len() < 1 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
