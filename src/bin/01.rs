advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());
        right.push(items.next().unwrap().parse::<i32>().unwrap());
    }
    left.sort();
    right.sort();

    let agg: i32 = std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum();
    Some(agg as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<usize>().unwrap());
        right.push(items.next().unwrap().parse::<usize>().unwrap());
    }
    left.sort();
    right.sort();

    let result: usize = left
        .iter()
        .map(|number| number * right.iter().filter(|num| &number == num).count())
        .sum();

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
