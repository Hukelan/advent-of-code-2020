use std::convert::TryInto;

pub fn day_01(input_lines: &[String]) -> (i64, i64) {
    let mut numbers: Vec<i64> = input_lines
        .iter()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    numbers.sort();

    let (part1_x, part1_y) = get_summands_from_list(2020, &numbers);
    let part1 = part1_x * part1_y;

    let mut skip: Vec<usize> = Vec::new();
    let v = get_summands_from_list_naive(3, 2020, &numbers, &mut skip);
    let part2 = v[0] * v[1] * v[2];

    (part1, part2)
}

fn get_summands_from_list(desired_sum: i64, numbers: &Vec<i64>) -> (i64, i64) {
    if numbers.len() < 2 {
        panic!(
            "Numbers list must have greater than one entry, got {}.",
            numbers.len()
        )
    }

    let mut x_ind = 0;
    let mut y_ind = numbers.len() - 1;
    loop {
        let sum = numbers[x_ind] + numbers[y_ind];
        if sum == desired_sum {
            break (numbers[x_ind], numbers[y_ind]);
        } else if x_ind == y_ind {
            break (0, 0);
        } else if sum < desired_sum {
            x_ind += 1;
        } else if sum > desired_sum {
            y_ind -= 1;
        }
    }
}

fn get_summands_from_list_naive(
    summands: u64,
    desired_sum: i64,
    numbers: &Vec<i64>,
    skip: &mut Vec<usize>,
) -> Vec<i64> {
    if numbers.len() < summands.try_into().unwrap() {
        panic!(
            "Numbers list must have at least {} entries, but has {}.",
            summands,
            numbers.len()
        )
    }

    for (index, number) in numbers.iter().enumerate() {
        if skip.contains(&index) {
            continue;
        }

        if summands == 1 {
            if desired_sum - number == 0 {
                return vec![*number];
            }
        } else {
            // still need 2 or more summands
            skip.push(index);
            let mut part_result =
                get_summands_from_list_naive(summands - 1, desired_sum - number, numbers, skip);
            skip.pop();
            if !part_result.is_empty() {
                part_result.push(*number);
                return part_result;
            }
        }
    }

    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let mut numbers = vec![1721, 979, 366, 299, 675, 1456];
        numbers.sort();
        let mut skip: Vec<usize> = Vec::new();
        let mut v = get_summands_from_list_naive(2, 2020, &numbers, &mut skip);
        v.sort();
        assert_eq!(v[0], 299);
        assert_eq!(v[1], 1721);
    }

    #[test]
    #[should_panic(expected = "Numbers list must have at least 2 entries, but has 0.")]
    fn empty_list() {
        let numbers = vec![];
        let mut skip: Vec<usize> = Vec::new();
        let mut v = get_summands_from_list_naive(2, 2020, &numbers, &mut skip);
        v.sort();
        assert_eq!(v[0], 299);
        assert_eq!(v[1], 1721);
    }

    #[test]
    #[should_panic(expected = "Numbers list must have at least 2 entries, but has 1.")]
    fn single_value() {
        let mut numbers = vec![2020];
        numbers.sort();
        let mut skip: Vec<usize> = Vec::new();
        let mut v = get_summands_from_list_naive(2, 2020, &numbers, &mut skip);
        v.sort();
        assert_eq!(v[0], 299);
        assert_eq!(v[1], 1721);
    }

    #[test]
    fn no_correct_result() {
        let mut numbers = vec![1, 2, 3, 4, 5, 6];
        numbers.sort();
        let mut skip: Vec<usize> = Vec::new();
        let mut v = get_summands_from_list_naive(2, 2020, &numbers, &mut skip);
        v.sort();
        assert!(v.is_empty());
    }
}

