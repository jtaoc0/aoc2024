advent_of_code::solution!(2);

fn indubitably_increasing(levels: &[&str]) -> bool {
    for i in 0..levels.len()-1 {
        let curr: i32 = levels[i].parse::<i32>().unwrap();
        let the_one_after_curr: i32 = levels[i+1].parse::<i32>().unwrap();

        if the_one_after_curr <= curr {
            return false;
        }
    }
    return true;
}

fn definitely_decreasing(levels: &[&str]) -> bool {
    for i in 0..levels.len()-1 {
        let curr: i32 = levels[i].parse::<i32>().unwrap();
        let the_one_after_curr: i32 = levels[i+1].parse::<i32>().unwrap();

        if the_one_after_curr >= curr {
            return false;
        }
    }
    return true;
}

fn within_acceptable_boundaries(levels: &[&str]) -> bool {
    for i in 0..levels.len()-1 {
        let curr: i32 = levels[i].parse::<i32>().unwrap();
        let the_one_after_curr: i32 = levels[i+1].parse::<i32>().unwrap();
        if (curr - the_one_after_curr).abs() > 3 {
            return false;
        }
    }
    return true;
}

fn levels_are_indubitably_increasing_or_definitely_decreasing_within_acceptable_boundaries(levels: &[&str]) -> bool {
    return (indubitably_increasing(levels) || definitely_decreasing(levels)) && within_acceptable_boundaries(levels)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut this_variable_contains_the_number_of_safe_reports: u32 = 0;
    input.lines().for_each(|line: &str| {
        let levels_from_the_report_for_this_line: Vec<&str> = line.split_whitespace().collect();
        if levels_are_indubitably_increasing_or_definitely_decreasing_within_acceptable_boundaries(&levels_from_the_report_for_this_line) {
            this_variable_contains_the_number_of_safe_reports += 1;
        }
    });

    return Some(this_variable_contains_the_number_of_safe_reports);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut this_variable_contains_the_number_of_safe_reports: u32 = 0;
    input.lines().for_each(|line: &str| {

        let mut we_got_em: bool = false;

        let levels_from_the_report_for_this_line: Vec<&str> = line.split_whitespace().collect();
        if levels_are_indubitably_increasing_or_definitely_decreasing_within_acceptable_boundaries(&levels_from_the_report_for_this_line) {
            we_got_em = true;
        }

        let mut adjusted_levels: &[&str];
        let mut temporary_removed_one: Option<Vec<&str>>;

        let mut i: usize = 0;
        
        while i < levels_from_the_report_for_this_line.len() && !we_got_em {
            if i == 0 {
                adjusted_levels = &levels_from_the_report_for_this_line[1..];
            }
            else if i == levels_from_the_report_for_this_line.len() - 1 {
                adjusted_levels = &levels_from_the_report_for_this_line[..i];
            }
            else {
                // def had to chatgpt how to do this combining vector slices thing
                temporary_removed_one = Some(levels_from_the_report_for_this_line[..i]
                    .iter()
                    .chain(&levels_from_the_report_for_this_line[i + 1..])
                    .copied()
                    .collect());
                adjusted_levels = temporary_removed_one.as_ref().unwrap();
            }
            
            if levels_are_indubitably_increasing_or_definitely_decreasing_within_acceptable_boundaries(adjusted_levels) {
                we_got_em = true;
            }
            i += 1;
        }
        if we_got_em {
            this_variable_contains_the_number_of_safe_reports += 1;
        }
    });

    return Some(this_variable_contains_the_number_of_safe_reports);
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
