advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut calibration: u32 = 0;
    for line in lines {
        let first_value = line.chars().find(char::is_ascii_digit).unwrap();
        let second_value = line.chars().rfind(char::is_ascii_digit).unwrap();
        let mut combined_value = String::from(first_value);
        combined_value.push(second_value);
        calibration = calibration + combined_value.parse::<u32>().unwrap();
    }
    return Some(calibration);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = String::from(input);

    input = input.replace("one", "1");
    input = input.replace("two", "2");
    input = input.replace("three", "3");
    input = input.replace("four", "4");
    input = input.replace("five", "5");
    input = input.replace("six", "6");
    input = input.replace("seven", "7");
    input = input.replace("eight", "8");
    input = input.replace("nine", "9");
    let lines = input.lines();
    let mut calibration: u32 = 0;
    for line in lines {
        let first_value = line.chars().find(char::is_ascii_digit).unwrap();
        let second_value = line.chars().rfind(char::is_ascii_digit).unwrap();
        let mut combined_value = String::from(first_value);
        combined_value.push(second_value);
        calibration = calibration + combined_value.parse::<u32>().unwrap();
    }
    return Some(calibration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, 281);
    }
}
