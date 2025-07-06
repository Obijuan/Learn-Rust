use std::str::FromStr;
use std::env;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
                     .expect("Error parsin argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}",
              numbers, d);
}

#[allow(dead_code)]
fn main1() {

    let a = 14;
    let b = 7;

    println!("Calcular el Máximo Común divisor");
    println!("Numeros: {} y {}",a,b);

    let n = gcd(a,b);

    println!("Resultado: {}", n);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0); 
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 
                   3 * 7 * 11 * 13 * 19),
                3 * 11);
}

