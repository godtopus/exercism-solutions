pub fn sum_of_multiples(n: i32, v: &Vec<i32>) -> i32 {
    (1..n).filter(|i| v.into_iter().any(|a| i % a == 0)).sum()

    //v.into_iter().fold(0, |a, &b| a + match b < n { true => (b..n).filter(|a| a % n == 0).sum(), false => 0 })
}