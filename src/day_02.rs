use std::convert::TryInto;

pub fn day_02(input_lines: &[String]) -> (i64, i64) {
    let password_lines: Vec<PasswordLine> = input_lines
        .iter()
        .map(|s| build_password_line(s.to_string()))
        .collect();

    let valid_passwords1 = password_lines
        .iter()
        .filter(|pl| is_password_valid1(&pl.password, &pl.policy))
        .count();

    let valid_passwords2 = password_lines
        .iter()
        .filter(|pl| is_password_valid2(&pl.password, &pl.policy))
        .count();

    (valid_passwords1.try_into().unwrap(), valid_passwords2.try_into().unwrap())
}

#[derive(Debug)]
struct PasswordLine {
    policy: PasswordPolicy,
    password: String,
}

#[derive(Debug)]
struct PasswordPolicy {
    min_occurance: u8,
    max_occurance: u8,
    character: char,
}

fn build_password_line(input_line: String) -> PasswordLine {
    let split: Vec<&str> = input_line
        .split(": ")
        // .map(std::string::ToString::to_string)
        .collect();
    PasswordLine {
        policy: build_password_policy(split[0]),
        password: split[1].to_string(),
    }
}

fn build_password_policy(input_part: &str) -> PasswordPolicy {
    let split1: Vec<String> = input_part
        .split(" ")
        .map(std::string::ToString::to_string)
        .collect();
    let split2: Vec<String> = split1[0]
        .split("-")
        .map(std::string::ToString::to_string)
        .collect();
    PasswordPolicy {
        min_occurance: split2[0].parse().unwrap(),
        max_occurance: split2[1].parse().unwrap(),
        character: split1[1].parse().unwrap(),
    }
}

fn is_password_valid1(password: &String, policy: &PasswordPolicy) -> bool {
    let char_occurances: u8 = password
        .chars()
        .filter(|c| *c == policy.character)
        .count()
        .try_into()
        .unwrap();
    policy.min_occurance <= char_occurances && char_occurances <= policy.max_occurance
}

fn is_password_valid2(password: &String, policy: &PasswordPolicy) -> bool {
    let pos1 = usize::from(policy.min_occurance);
    let pos2 = usize::from(policy.max_occurance);

    let valid_min =
        password.len() >= pos1 && password.chars().nth(pos1-1).unwrap() == policy.character;
    let valid_max =
        password.len() >= pos2 && password.chars().nth(pos2-1).unwrap() == policy.character;

    valid_min ^ valid_max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        let input_lines: Vec<String> = example
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        let (part1, part2) = day_02(&input_lines);
        assert_eq!(part1, 2);
        assert_eq!(part2, 1);
    }

    #[test]
    fn test_password_lines() {
        let pl1 = build_password_line(String::from("1-3 a: abcde"));
        assert_eq!(pl1.password, "abcde");
        assert_eq!(pl1.policy.min_occurance, 1);
        assert_eq!(pl1.policy.max_occurance, 3);
        assert_eq!(pl1.policy.character, 'a');

        let pl1 = build_password_line(String::from("1-3 b: cdefg"));
        assert_eq!(pl1.password, "cdefg");
        assert_eq!(pl1.policy.min_occurance, 1);
        assert_eq!(pl1.policy.max_occurance, 3);
        assert_eq!(pl1.policy.character, 'b');

        let pl1 = build_password_line(String::from("2-9 c: ccccccccc"));
        assert_eq!(pl1.password, "ccccccccc");
        assert_eq!(pl1.policy.min_occurance, 2);
        assert_eq!(pl1.policy.max_occurance, 9);
        assert_eq!(pl1.policy.character, 'c');
    }

    #[test]
    fn test_is_password_valid1() {
        let policy1 = PasswordPolicy {
            min_occurance: 1,
            max_occurance: 3,
            character: 'a',
        };
        assert!(is_password_valid1(&String::from("abcde"), &policy1));

        let policy2 = PasswordPolicy {
            min_occurance: 1,
            max_occurance: 3,
            character: 'b',
        };
        assert!(!is_password_valid1(&String::from("cdefg"), &policy2));

        let policy3 = PasswordPolicy {
            min_occurance: 2,
            max_occurance: 9,
            character: 'c',
        };
        assert!(is_password_valid1(&String::from("ccccccccc"), &policy3));
    }

    #[test]
    fn test_is_password_valid2() {
        let policy1 = PasswordPolicy {
            min_occurance: 1,
            max_occurance: 3,
            character: 'a',
        };
        assert!(is_password_valid2(&String::from("abcde"), &policy1));

        let policy2 = PasswordPolicy {
            min_occurance: 1,
            max_occurance: 3,
            character: 'b',
        };
        assert!(!is_password_valid2(&String::from("cdefg"), &policy2));

        let policy3 = PasswordPolicy {
            min_occurance: 2,
            max_occurance: 9,
            character: 'c',
        };
        assert!(!is_password_valid2(&String::from("ccccccccc"), &policy3));
    }
}
