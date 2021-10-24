fn main() {
    // https://projecteuler.net/problem=1
    // If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
    // Find the sum of all the multiples of 3 or 5 below 1000.
    println!("Sum: {}", sum_of_3s_and_5s(1000));
}

fn sum_of_3s_and_5s(upper_bound: u32) -> u32 {
    (1..upper_bound).filter(|x| x%3==0||x%5==0).sum()
}

#[cfg(test)]
mod tests {
    use crate::sum_of_3s_and_5s;

    #[test]
    fn basic_test() {
        assert_eq!(sum_of_3s_and_5s(10), 23)
    }
}