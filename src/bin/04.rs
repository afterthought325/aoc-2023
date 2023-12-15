use std::collections::HashSet;

advent_of_code::solution!(4);

struct Card {
    //number: u32,
    winning: HashSet<u32>,
    yours: HashSet<u32>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let cards = lines_to_cards(&lines);
    let mut total: u32 = 0;
    for card in cards {
        println!("Winning #s:{:?}", card.winning);
        println!("Your #s:{:?}", card.yours);
        let matching: Vec<&u32> = card.winning.intersection(&card.yours).collect();
        let num_same: u32 = matching.iter().count().try_into().unwrap();
        let points = if num_same > 1 {
            2_u32.pow(num_same - 1)
        } else {
            num_same
        };
        println!("Matching: {:?}", matching);
        println!("#: {}", num_same);
        println!("Points: {}\n", points);
        total += points;
    }
    Some(total)
}

fn lines_to_cards(lines: &Vec<&str>) -> Vec<Card> {
    lines
        .iter()
        .map(|x| {
            let (card, nums) = x.split_once(": ").unwrap();
            let card_number: u32 = card
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .expect("Error Decoding Card #");
            let (win, yours) = nums.split_once(" | ").unwrap();
            let win: HashSet<u32> = win
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let yours: HashSet<u32> = yours
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            Card {
                //number: card_number,
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 30);
    }
}
