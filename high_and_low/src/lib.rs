fn high_and_low(numbers: &str) -> String {
    let mut int_numbers: Vec<i32> = numbers.split(' ').map(|x| x.parse()).flatten().collect();
    int_numbers.sort();
    let max = int_numbers.last().unwrap();
    let min = int_numbers.first().unwrap();
    format!("{} {}", max, min)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_test_1() {
        assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
    }

    #[test]
    fn example_test_2() {
        assert_eq!("3 1", high_and_low("1 2 3"));
    }
}
