extern crate time;
use time::PreciseTime;

fn sieve(n: u32) {
    let mut primes = Vec::new();
    primes.push(2);
    for i in 2..(n + 1) {
        if !primes.contains(&i) && i % 2 != 0 {
            println!("{}", i);
            for j in ((i * i)..n + 1).step_by(i as usize) {
                primes.push(j)
            }
        }
    }
}

fn main() {
    println!(
        "Super duper Roby Sieve of Eratosthenes in Rust, taking time for a 60000 set of numbers!"
    );
    let start = PreciseTime::now();
    sieve(60000);
    let end = PreciseTime::now();
    println!("WOW! it took {} seconds.", start.to(end));
}
