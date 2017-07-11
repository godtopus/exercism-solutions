pub fn primes_up_to(n: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = (2 .. n + 1).collect();
    let mut pos = 0;

    while pos != primes.len() {
        let prime = primes[pos];
        primes.retain(|&i| i == prime || i % prime != 0);
        pos += 1;
    }

    primes
}