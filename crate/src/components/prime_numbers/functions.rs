// calculate prime numbers
pub fn prime_numbers(n: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; n as usize];
    for i in 2..n {
        if is_prime[i as usize] {
            primes.push(i);
            let mut j = i * i;
            while j < n {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }
    primes
}
