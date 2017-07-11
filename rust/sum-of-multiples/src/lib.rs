pub fn sum_of_multiples(n: i32, v: &Vec<i32>) -> i32 {
    (1..n).filter(|i| v.into_iter().any(|a| i % a == 0)).sum()
}