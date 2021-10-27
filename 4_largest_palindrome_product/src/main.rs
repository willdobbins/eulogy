use std::collections::HashMap;

fn main() {
    //Problem 4
    //A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
    //Find the largest palindrome made from the product of two 3-digit numbers.
    //Check range 900..1000.rev() for x and y.
    let mut skip_list = HashMap::new();
    let mut best_set: (u32, u32) = (0, 0);
    let mut calc_count = 0;
    for x in (100..1000).rev() {
        for y in (100..1000).rev() {
            let skip_key = make_skip_key(x, y);

            if skip_list.contains_key(&skip_key) { // if we pre-calc'd, skip.
                continue;
            }

            if (best_set.0 * best_set.1 < x * y) && is_palindromic_number(x * y) {
                println!("({},{}): {}\t{}", x, y, x*y, calc_count);

                best_set = skip_key
            }
            skip_list.insert(skip_key, x * y);
            calc_count = calc_count + 1;
        }
    }
    println!("{} total calcs", calc_count);
    println!("{} * {} = {}", best_set.0, best_set.1, skip_list.get(&best_set).unwrap());

}

fn make_skip_key(x: u32, y: u32) -> (u32, u32) {
    match x < y {
        true => { (x, y) }
        false => { (y, x) }
    }
}

fn is_palindromic_number(number: u32) -> bool {
    // take first half of string reading forward and back half of string reading forward,
    // and return front_char == back_char.
    let parsed_number = number.to_string();
    parsed_number.chars().take(parsed_number.len() / 2)
        .zip(parsed_number.chars().rev().take(parsed_number.len() / 2))
        .all(|(x, y)| x == y)
}

#[cfg(test)]
mod tests {
    use crate::{is_palindromic_number, make_skip_key};

    #[test]
    fn test_make_skip_key() {
        assert_eq!(make_skip_key(100, 101), (100, 101));
        assert_eq!(make_skip_key(999, 100), (100, 999));
    }

    #[test]
    fn test_is_palindromic_number() {
        assert_eq!(is_palindromic_number(101), true);
        assert_eq!(is_palindromic_number(100), false);
        assert_eq!(is_palindromic_number(1001), true);
    }
}