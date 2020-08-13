use std::str;

#[aoc(day4, part1)]
pub fn valid_passwords(input: &str) -> usize {
    count_passwords(input, has_repeated)
}

#[aoc(day4, part2)]
pub fn valid_passwords_part2(input: &str) -> usize {
    count_passwords(input, has_double)
}

pub fn count_passwords<F>(input: &str, is_valid: F) -> usize
where
    F: Fn(&Vec<usize>) -> bool,
{
    let mut valid: usize = 0;
    // Use one less to remove initial case in loop
    let mut digits = get_digits(input[0..6].parse::<usize>().unwrap() - 1);
    let max = get_digits(input[7..13].parse::<usize>().unwrap());

    loop {
        increment(&mut digits);
        make_increasing(&mut digits);
        if digits > max {
            break;
        }
        if is_valid(&digits) {
            valid += 1;
        }
    }
    valid
}

fn get_digits(mut d: usize) -> Vec<usize> {
    let mut digits = Vec::with_capacity(6);
    for _ in 0..6 {
        digits.push(d % 10);
        d /= 10;
    }
    digits.reverse();
    digits
}

fn increment(digits: &mut Vec<usize>) {
    let mut i = 5;
    loop {
        digits[i] = digits[i] + 1;
        if digits[i] < 10 {
            break;
        }
        digits[i] = 0;
        i = i - 1;
    }
}

fn make_increasing(digits: &mut Vec<usize>) {
    for i in 1..digits.len() {
        if digits[i] < digits[i - 1] {
            digits[i] = digits[i - 1];
        }
    }
}

fn has_double(digits: &Vec<usize>) -> bool {
    let mut count = 0;
    let mut last = digits[0];
    for i in 1..digits.len() {
        if digits[i] == last {
            count = count + 1;
        } else {
            if count == 1 {
                return true;
            }
            count = 0;
            last = digits[i];
        }
    }
    count == 1
}

fn has_repeated(digits: &Vec<usize>) -> bool {
    digits
        .windows(2)
        .fold(false, |acc, pair| acc || pair[0] == pair[1])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_digits() {
        assert_eq!(get_digits(5), vec![0, 0, 0, 0, 0, 5]);
        assert_eq!(get_digits(34567), vec![0, 3, 4, 5, 6, 7]);
        assert_eq!(get_digits(234567), vec![2, 3, 4, 5, 6, 7]);
        assert_eq!(get_digits(1234567), vec![2, 3, 4, 5, 6, 7]);
    }

    macro_rules! assert_f {
        ($action:tt, $initial:expr, $expected:expr) => {
            let mut value = $initial;
            $action(&mut value);
            assert_eq!(value, $expected);
        };
    }

    #[test]
    fn test_increment() {
        assert_f!(increment, vec![1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 7]);
        assert_f!(increment, vec![1, 2, 3, 4, 5, 9], vec![1, 2, 3, 4, 6, 0]);
        assert_f!(increment, vec![1, 2, 3, 4, 9, 9], vec![1, 2, 3, 5, 0, 0]);
    }

    #[test]
    fn test_enforce_increasing() {
        assert_f!(
            make_increasing,
            vec![1, 2, 3, 4, 5, 6],
            vec![1, 2, 3, 4, 5, 6]
        );
        assert_f!(
            make_increasing,
            vec![1, 2, 3, 4, 9, 6],
            vec![1, 2, 3, 4, 9, 9]
        );
        assert_f!(
            make_increasing,
            vec![1, 2, 5, 4, 3, 6],
            vec![1, 2, 5, 5, 5, 6]
        );
    }

    #[test]
    fn test_doubles() {
        assert_eq!(has_double(&vec![1, 2, 3]), false);
        assert_eq!(has_double(&vec![1, 2, 2]), true);
        assert_eq!(has_double(&vec![2, 2, 2]), false);
        assert_eq!(has_double(&vec![1, 1, 1, 1, 2, 2]), true);
        assert_eq!(has_double(&vec![1, 1, 2, 2, 3, 3]), true);
        assert_eq!(has_double(&vec![1, 1, 1, 2, 2, 3]), true);
    }

    #[test]
    fn test_repeated() {
        assert_eq!(has_repeated(&vec![1, 2, 3]), false);
        assert_eq!(has_repeated(&vec![1, 2, 2]), true);
        assert_eq!(has_repeated(&vec![1, 1, 2]), true);
        assert_eq!(has_repeated(&vec![1, 2, 1]), false);
    }
}
