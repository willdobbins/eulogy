use std::env;

fn main() {
    //    Problem 3
    //    The prime factors of 13195 are 5, 7, 13 and 29.
    //    What is the largest prime factor of the number 600851475143 ?

    // flow:
    // given number, we need to check primes range 2..target #.
    // sieve?
    let args: Vec<String> = env::args().collect();
    let target_number = &args[1].parse::<u64>().unwrap();
    println!("Getting largest prime for {}", target_number);
    let primes = get_primes(*target_number);
    println!("Largest Prime: {}", primes.last().unwrap());

}

fn get_primes(number: u64) -> Vec<u64> {
    println!("Checking primes from 2..={}", number);
    let mut primes:Vec<u64> = Vec::new();
    let mut term:u64 = 2;
    let mut subject = number;
    while term <= subject {
        if subject % term == 0 {
            println!("{} divisible by {}", subject, term);
            subject /= term;
            if !primes.contains(&term) {
                primes.push(term);
                println!("Primes: {:?}", primes);
            }
        }
        if subject % term != 0 {
            term+=1;
        }
    }
    primes
}

#[cfg(test)]
mod tests {
    use crate::get_primes;

    #[test]
    fn test_basic() {
        assert_eq!(get_primes(13195), [5, 7, 13, 29]);
        assert_eq!(get_primes(15), [3,5]);
    }
}
