#![feature(slice_group_by)]

use counter::Counter;
advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut hands = Vec::<Hand>::new();
    for line in lines {
        let (card_str, bid_str) = line.split_once(' ').unwrap();
        let cards = convert_hand(card_str, false);
        let bid = bid_str.parse::<u32>().unwrap();
        let hand_type = derive_hand_type(&cards);
        let hand = Hand {
            hand_type,
            cards,
            bid,
        };
        hands.push(hand);
    }
    hands.sort();

    let mut result: u32 = 0;
    for (index, hand) in hands.iter().enumerate() {
        result += (u32::try_from(index).unwrap() + 1) * hand.bid;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut hands = Vec::<Hand>::new();
    for line in lines {
        let (card_str, bid_str) = line.split_once(' ').unwrap();
        let cards = convert_hand(card_str, true);
        let bid = bid_str.parse::<u32>().unwrap();
        let hand_type = derive_hand_type(&cards);
        let hand = Hand {
            hand_type,
            cards,
            bid,
        };
        hands.push(hand);
    }
    hands.sort();

    let mut result: u32 = 0;
    for (index, hand) in hands.iter().enumerate() {
        result += (u32::try_from(index).unwrap() + 1) * hand.bid;
    }

    Some(result)
}

fn swap_for_value(input: char, joker: bool) -> u32 {
    if input.is_numeric() {
        input.to_digit(10).unwrap()
    } else {
        match input {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => {
                if joker {
                    1
                } else {
                    11
                }
            }
            'T' => 10,
            _ => 0,
        }
    }
}

fn convert_hand(input: &str, joker: bool) -> Vec<u32> {
    input.chars().map(|x| swap_for_value(x, joker)).collect()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfKind = 4,
    FullHouse = 5,
    FourOfKind = 6,
    FiveOfKind = 7,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Hand {
    hand_type: HandType,
    cards: Vec<u32>,
    bid: u32,
}

fn derive_hand_type(input: &[u32]) -> HandType {
    let hand = input.to_owned();
    let mut groups = hand.iter().collect::<Counter<_, u32>>();
    let jokers = groups.remove(&1).unwrap_or(0u32);

    // get a vector of tuples sorted by most common card to least common card and their corresponding amounts;
    let most_common: Vec<(&u32, u32)> = groups.most_common_ordered();

    match jokers {
        5 => HandType::FiveOfKind,
        4 => HandType::FiveOfKind, // any card + 4 Jokers == FourOfKind
        3 => {
            match most_common[0].1 {
                2 => HandType::FiveOfKind, // OnePair + 3 jokers == FiveOfKind
                1 => HandType::FourOfKind, // any card + 3 jokers == FourOfKind
                _ => HandType::HighCard,
            }
        }
        2 => {
            match most_common[0].1 {
                3 => HandType::FiveOfKind,  // 3OfKind + 2 jokers == FiveOfKind
                2 => HandType::FourOfKind,  // OnePair + 2 jokers == Four of a Kind
                1 => HandType::ThreeOfKind, // Means there must be 3 unique cards left
                _ => HandType::HighCard,
            }
        }
        1 => {
            match most_common[0].1 {
                4 => HandType::FiveOfKind, // FourOfKind + 1 Joker == FiveOfKind
                3 => HandType::FourOfKind, // ThreeOfKind + 1 Joker == FourOfKind
                2 => {
                    if most_common.len() == 2 {
                        // 2Pair + 1 Joker == FullHouse
                        HandType::FullHouse
                    } else {
                        // OnePair + 1 Joker ==  ThreeOfKind
                        HandType::ThreeOfKind
                    }
                }
                1 => HandType::OnePair,
                _ => HandType::HighCard,
            }
        }
        _ => match most_common[0].1 {
            5 => HandType::FiveOfKind,
            4 => HandType::FourOfKind,
            3 => {
                if most_common[1].1 == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfKind
                }
            }
            2 => {
                if most_common[1].1 == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_hands() {
        let hand1 = Hand {
            hand_type: HandType::FiveOfKind,
            cards: vec![2, 2, 2, 2, 2],
            bid: 122,
        };
        let hand2 = Hand {
            hand_type: HandType::FiveOfKind,
            cards: vec![3, 3, 3, 3, 3],
            bid: 222,
        };
        let hand3 = Hand {
            hand_type: HandType::FourOfKind,
            cards: vec![4, 3, 3, 3, 3],
            bid: 222,
        };
        let hand4 = Hand {
            hand_type: HandType::FiveOfKind,
            cards: vec![2, 2, 2, 2, 2],
            bid: 122,
        };
        assert_eq!(hand1.cmp(&hand2), Ordering::Less);
        assert_eq!(hand2.cmp(&hand3), Ordering::Greater);
        assert_eq!(hand1.cmp(&hand4), Ordering::Equal);
    }

    #[test]
    fn test_sort_hands() {
        let hand1 = Hand {
            hand_type: HandType::FiveOfKind,
            cards: vec![2, 2, 2, 2, 2],
            bid: 122,
        };
        let hand2 = Hand {
            hand_type: HandType::FiveOfKind,
            cards: vec![3, 3, 3, 3, 3],
            bid: 222,
        };
        let hand3 = Hand {
            hand_type: HandType::FourOfKind,
            cards: vec![4, 3, 3, 3, 3],
            bid: 222,
        };
        let hand4 = Hand {
            hand_type: HandType::HighCard,
            cards: vec![2, 3, 4, 5, 6],
            bid: 122,
        };

        let mut hands = vec![hand4.clone(), hand2.clone(), hand3.clone(), hand1.clone()];
        let expected = vec![hand4, hand3, hand1, hand2];
        hands.sort();
        assert_eq!(expected, hands);
    }

    #[test]
    fn test_convert_hand() {
        // Standard
        let hand = "AKQJT98765432";
        let converted_hand = vec![14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2];
        assert_eq!(convert_hand(hand, false), converted_hand);
        // Joker
        let joker_hand = "AKQT98765432J";
        let converted_joker_hand = vec![14, 13, 12, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(convert_hand(joker_hand, true), converted_joker_hand);
    }

    #[test]
    fn test_hand_type() {
        // Standard
        let five_kind: Vec<u32> = vec![14, 14, 14, 14, 14];
        let four_kind: Vec<u32> = vec![14, 14, 14, 14, 5];
        let full_house: Vec<u32> = vec![14, 14, 14, 9, 9];
        let three_kind: Vec<u32> = vec![14, 14, 14, 2, 9];
        let two_pair: Vec<u32> = vec![2, 2, 3, 3, 4];
        let one_pair: Vec<u32> = vec![2, 2, 3, 4, 5];
        let high_card: Vec<u32> = vec![2, 3, 4, 5, 6];
        assert_eq!(derive_hand_type(&five_kind), HandType::FiveOfKind);
        assert_eq!(derive_hand_type(&four_kind), HandType::FourOfKind);
        assert_eq!(derive_hand_type(&full_house), HandType::FullHouse);
        assert_eq!(derive_hand_type(&three_kind), HandType::ThreeOfKind);
        assert_eq!(derive_hand_type(&two_pair), HandType::TwoPair);
        assert_eq!(derive_hand_type(&one_pair), HandType::OnePair);
        assert_eq!(derive_hand_type(&high_card), HandType::HighCard);
        // Joker
        let five_joker: Vec<u32> = vec![1, 1, 1, 1, 1];
        let four_joker: Vec<u32> = vec![2, 1, 1, 1, 1];
        let five_kind: Vec<u32> = vec![1, 14, 14, 14, 14];
        let four_kind: Vec<u32> = vec![1, 14, 14, 14, 5];
        let full_house: Vec<u32> = vec![1, 14, 14, 9, 9];
        let three_kind: Vec<u32> = vec![1, 14, 14, 2, 9];
        let one_pair: Vec<u32> = vec![1, 2, 3, 4, 5];
        assert_eq!(derive_hand_type(&five_joker), HandType::FiveOfKind);
        assert_eq!(derive_hand_type(&four_joker), HandType::FiveOfKind);
        assert_eq!(derive_hand_type(&five_kind), HandType::FiveOfKind);
        assert_eq!(derive_hand_type(&four_kind), HandType::FourOfKind);
        assert_eq!(derive_hand_type(&full_house), HandType::FullHouse);
        assert_eq!(derive_hand_type(&three_kind), HandType::ThreeOfKind);
        assert_eq!(derive_hand_type(&two_pair), HandType::TwoPair);
        assert_eq!(derive_hand_type(&one_pair), HandType::OnePair);
        assert_eq!(derive_hand_type(&high_card), HandType::HighCard);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
