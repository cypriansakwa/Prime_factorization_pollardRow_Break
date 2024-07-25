use num_bigint::BigUint;
use num_traits::{One, Zero};
use num_integer::Integer;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Instant;

fn pollards_rho(n: &BigUint) -> BigUint {
    let mut x = BigUint::from(2u32);
    let mut y = BigUint::from(2u32);
    let mut d = BigUint::one();
    let one = BigUint::one();
    let f = |z: &BigUint| (z * z + &one) % n;
    let mut iteration_count = 0;
    let iteration_limit = 10_000_000;  // Set a reasonable iteration limit

    while d == BigUint::one() {
        if iteration_count > iteration_limit {
            panic!("Pollard's rho algorithm exceeded iteration limit. Potential infinite loop.");
        }
        x = f(&x);
        y = f(&f(&y));
        let abs_diff = if &x > &y { &x - &y } else { &y - &x };
        d = abs_diff.gcd(n);

        iteration_count += 1;

        if d != BigUint::one() && d != *n {
            break;
        }
    }

    d
}

fn trial_division(n: &BigUint, limit: &BigUint) -> Option<BigUint> {
    let mut i = BigUint::from(2u32);
    while &i <= limit {
        if n % &i == BigUint::zero() {
            return Some(i);
        }
        i += BigUint::one();
    }
    None
}

fn prime_factors(n: &BigUint) -> HashMap<BigUint, u32> {
    let mut factors = HashMap::new();
    let mut current = n.clone();
    let sqrt_limit = sqrt(&current);

    if let Some(factor) = trial_division(&current, &sqrt_limit) {
        while (&current % &factor).is_zero() {
            *factors.entry(factor.clone()).or_insert(0) += 1;
            current /= &factor;
            println!("Found small factor: {} - Remaining number: {}", factor, current);  // Log found factor
        }
    }

    while current > BigUint::one() {
        if is_prime(&current) {
            *factors.entry(current.clone()).or_insert(0) += 1;
            println!("Found prime factor: {} - Remaining number: {}", current, BigUint::one());
            break;
        }

        let factor = pollards_rho(&current);
        let mut count = 0;
        while (&current % &factor).is_zero() {
            current /= &factor;
            count += 1;
        }

        *factors.entry(factor.clone()).or_insert(0) += count;
        println!("Found factor: {} (count: {}) - Remaining number: {}", factor, count, current);  // Log found factor
    }

    factors
}

fn is_prime(n: &BigUint) -> bool {
    if n <= &BigUint::one() {
        return false;
    }
    let two = BigUint::from(2u32);
    if n == &two {
        return true;
    }
    if n.is_even() {
        return false;
    }

    let mut i = BigUint::from(3u32);
    let limit = sqrt(n) + &BigUint::one();
    while i < limit {
        if n % &i == BigUint::zero() {
            return false;
        }
        i += &two;
    }
    true
}

fn sqrt(n: &BigUint) -> BigUint {
    let mut x0: BigUint = n.clone();
    let mut x1: BigUint = (n >> 1) + BigUint::one();
    while x1 < x0 {
        x0 = x1.clone();
        x1 = (n / &x1 + &x1) >> 1;
    }
    x0
}

fn main() {
    let number_str ="31500";
    let n = BigUint::from_str(number_str).unwrap();

    println!("Starting factorization for: {}", number_str);
    let start = Instant::now();

    let factors = prime_factors(&n);

    let duration = start.elapsed();

    println!("Prime factors of {}:", number_str);
    for (factor, count) in factors {
        println!("{}^{}", factor, count);
    }

    println!("Time taken: {:?}", duration);
}


