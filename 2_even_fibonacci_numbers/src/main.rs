fn main() {
    //Each new term in the Fibonacci sequence is generated by adding the previous two terms.
    // By starting with 1 and 2, the first 10 terms will be:
    //    1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
    //By considering the terms in the Fibonacci sequence whose values do not exceed four million,
    // find the sum of the even-valued terms.

    // Flow:
    // 1. Generate Fibonacci series until we hit 4mil+ term.
    let mut term: u32 = 2;
    let mut prev_term: u32 = 1;
    let mut swap:u32;
    let mut even_sum: u32 = 0;
    while term < 4000000 {
        if term%2 ==0 {
            even_sum += term;
        }
        swap = term;
        term += prev_term;
        prev_term = swap;
    }
    println!("{}", even_sum);
}