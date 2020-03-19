// iterators4.rs



// https://en.wikipedia.org/wiki/Factorial
// In mathematics, the factorial of a positive integer n, denoted by n!, is the product of all positive integers less than or equal to n:
// For example,

// 5! = 5 x 4 x 3 x 2 x 1 = 120


pub fn factorial(num: u64) -> u64 {
    // Complete this function to return factorial of num
    // Do not use:
    // - return
    // For extra fun don't use:
    // - imperative style loops (for, while)
    // - additional variables
    // For the most fun don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    let mut f: u64 = 1;

    
    for n in 1..=num {
        f =  f * n
    }

    f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
