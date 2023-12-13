use std::collections::HashSet;

advent_of_code::solution!(4);

struct Card {
    number: u32,
    winning: HashSet<_>,
    yours: HashSet<_>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split("\n").collect();

    None
}

fn lines_to_cards(lines: &Vec<&str>) -> Vec<Card> {
    lines
        .iter()
        .map(|x| {
            let (card, nums) = x.split_once(" ").unwrap().1.split_once(":").unwrap();
            let card_number: u32 = card.parse().expect("Error Decoding Card #");
            let (win, yours) = nums.split_once("|").unwrap();
            let win: Vec<u32> = win.trim().split(" ").map(|x| x.parse::<u32>()).collect();
            let yours: Vec<u32> = yours.trim().split(" ").map(|x| x.parse::<u32>()).collect();
            Card {
                number: card_number,
                winning: win,
                yours: yours,
            }
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
