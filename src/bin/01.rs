use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split("\n").collect();
    let num_lines: usize = lines.len();
    let mut left_nums: Vec<i32> = Vec::new();
    let mut right_nums: Vec<i32> = Vec::new();
    for line in lines {
        let mut nums = line.split("   ");
        let num1: i32 = nums.next().unwrap().parse().unwrap();
        let num2: i32 = nums.next().unwrap().parse().unwrap();
        left_nums.push(num1);
        right_nums.push(num2);
    }
    left_nums.sort();
    right_nums.sort();
    let mut answer: i32 = 0;
    for i in 0..num_lines {
        answer = answer + (left_nums[i] - right_nums [i]).abs()
    }

    println!("FINAL ANSWER = {}", answer);
    return Some(answer as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut left_nums: Vec<i32> = Vec::new();
    let mut freqs: HashMap<i32, i32> = HashMap::new();
    for line in lines {
        let mut nums = line.split("   ");
        let num1: i32 = nums.next().unwrap().parse().unwrap();
        let num2: i32 = nums.next().unwrap().parse().unwrap();
        left_nums.push(num1);
        *freqs.entry(num2).or_insert(0) += 1;
    }
    left_nums.sort();
    let mut answer: i32 = 0;
    for num in left_nums {
        if freqs.contains_key(&num) {
            answer = answer + (num * freqs[&num])
        }
    }

    println!("FINAL ANSWER = {}", answer);
    return Some(answer as u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
