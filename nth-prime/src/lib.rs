fn is_prime (number: u32, got_primes: & Vec<u32>) -> bool {
    for prime in got_primes {
        if number % prime == 0 { 
            return false 
        };
    }
    return true;
}

pub fn nth(nth_prime_to_get: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2];
    let mut i: u32 = 2;
    loop {
        if primes.len() >= ((nth_prime_to_get + 1) as usize) {
            return primes[nth_prime_to_get as usize];
        }
        if is_prime(i, &primes) {
            println!("is prime {}", i);
            primes.push(i);   
        }
        i += 1;
    }
}
