// use std::convert::TryInto;

pub fn day_03(input_lines: &[String]) -> (i64, i64) {
    let slope: Vec<String> = input_lines.iter().map(std::string::ToString::to_string).collect();
    let slope11 = count_trees_on_path(&slope, 1, 1);
    let slope31 = count_trees_on_path(&slope, 3, 1);
    let slope51 = count_trees_on_path(&slope, 5, 1);
    let slope71 = count_trees_on_path(&slope, 7, 1);
    let slope12 = count_trees_on_path(&slope, 1, 2);
    (slope31, slope11*slope31*slope51*slope71*slope12)
}

fn is_tree_at_location(slope: &Vec<String>, x: usize, y: usize) -> bool {
    let x_remainder = x % slope[0].len();
    slope[y].chars().nth(x_remainder).unwrap() == '#'
}

fn count_trees_on_path(slope: &Vec<String>, dx: usize, dy: usize) -> i64 {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut trees: i64 = 0;
    while y < slope.len() {
        if is_tree_at_location(slope, x, y) {
            trees += 1;
        }
        x += dx;
        y += dy;
    }
    trees
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [&str; 11] = [
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];

    #[test]
    fn test_example() {
        let example: Vec<String> = EXAMPLE.iter().map(std::string::ToString::to_string).collect();
        let (part1, part2) = day_03(&example);
        assert_eq!(part1, 7);
        assert_eq!(part2, 336);
    }

    #[test]
    fn test_is_tree_at_location() {
        let example: Vec<String> = EXAMPLE.iter().map(std::string::ToString::to_string).collect();
        assert!(!is_tree_at_location(&example, 0, 0));
        assert!(!is_tree_at_location(&example, 3, 1));
        assert!(is_tree_at_location(&example, 6, 2));
        assert!(!is_tree_at_location(&example, 9, 3));
        assert!(is_tree_at_location(&example, 12, 4));
        assert!(is_tree_at_location(&example, 15, 5));
        assert!(!is_tree_at_location(&example, 18, 6));
        assert!(is_tree_at_location(&example, 21, 7));
        assert!(is_tree_at_location(&example, 24, 8));
        assert!(is_tree_at_location(&example, 27, 9));
        assert!(is_tree_at_location(&example, 30, 10));
    }
}
