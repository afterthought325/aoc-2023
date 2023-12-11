#![feature(slice_group_by)]

use std::cmp::Ordering;
advent_of_code::solution!(7);


pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str>= input.split("\n").collect();
    let mut hands = Vec::<Hand>::new();
    for line in lines {
        let (card_str, bid_str) = line.split_once(" ").unwrap();
        let cards = convert_hand(card_str, false);
        let bid = bid_str.parse::<u32>().unwrap();
        let hand_type = derive_hand_type(&cards, false);
        let hand = Hand {
            hand_type: hand_type,
            cards: cards,
            bid: bid
        };
        hands.push(hand);
    }
    hands.sort();

    let mut result:u32 = 0;
    for (index, hand) in hands.iter().enumerate() {
        result += (u32::try_from(index).unwrap() + 1) * hand.bid;
    }


    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str>= input.split("\n").collect();
    let mut hands = Vec::<Hand>::new();
    for line in lines {
        let (card_str, bid_str) = line.split_once(" ").unwrap();
        let cards = convert_hand(card_str, true);
        let bid = bid_str.parse::<u32>().unwrap();
        let hand_type = derive_hand_type(&cards, true);
        let hand = Hand {
            hand_type: hand_type,
            cards: cards,
            bid: bid
        };
        hands.push(hand);
    }
    hands.sort();

    let mut result:u32 = 0;
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
            'J' => if joker { 1 } else { 11 },
            'T' => 10,
             _  => 0,
        }
    }
}

fn convert_hand(input: &str, joker: bool) -> Vec<u32> {
    input.chars()
    .map(|x| swap_for_value(x, joker))
    .collect()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfKind = 4,
    FullHouse = 5,
    FourOfKind = 6,
    FiveOfKind = 7
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Hand {
    hand_type: HandType,
    cards: Vec<u32>,
    bid: u32,
}

fn derive_hand_type(input: &Vec<u32>, joker: bool) -> HandType {
    let mut hand = input.clone();
    // numbers are sorted least to greatest;
    hand.sort();
    // numbers are grouped into individual vectors
    let mut groups: Vec<_> = hand.group_by(|a,b| a == b).collect();
    // groups are sorted from largest to shortest;
    groups.sort_by(|a,b| {
    let a_len = a.len();
    let b_len = b.len();
    b_len.cmp(&a_len)
    });
    if groups.len() == 1 { // only 1 possibility
        return HandType::FiveOfKind;
    } else if groups.len() == 2 { // either FourOfKind or FullHouse
        if groups.iter().any(|&x| x.iter().any(|&x| x == 1u32)) && joker { //  One of the groups are jokers so is a five of a kind
            return HandType::FiveOfKind;
        }
        else if groups[0].len() == 4 {
            return HandType::FourOfKind;
        } else {
            return HandType::FullHouse;
        }
    } else if groups.len() == 3 { // either ThreeOfKind or TwoPair
        if groups.iter().any(|&x| x.iter().any(|&x| x == 1u32)) && joker { //  One of the groups are jokers so it's gonna get interesting
            if groups[0].len() == 2{
            } {
                return HandType::ThreeOfKind;
            } else {
                return HandType::TwoPair;
            }
            return HandType::FiveOfKind;
        }
        if groups[0].len() == 3 {
            return HandType::ThreeOfKind;
        } else {
            return HandType::TwoPair;
        }
    } else if groups.len() == 4 {
        return HandType::OnePair;
    } else {
        return HandType::HighCard;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_hands(){
        let hand1 = Hand {
            hand_type: HandType::FiveOfKind,
            cards: vec![2,2,2,2,2],
            bid: 122,
        };
        let hand2 = Hand {
            hand_type: HandType::FiveOfKind,
            cards: vec![3,3,3,3,3],
            bid: 222,
        };
        let hand3 = Hand {
            hand_type: HandType::FourOfKind,
            cards: vec![4,3,3,3,3],
            bid: 222,
        };
        let hand4 = Hand {
            hand_type: HandType::FiveOfKind,
            cards: vec![2,2,2,2,2],
            bid: 122,
        };
        assert_eq!(hand1.cmp(&hand2), Ordering::Less);
        assert_eq!(hand2.cmp(&hand3), Ordering::Greater);
        assert_eq!(hand1.cmp(&hand4), Ordering::Equal);
    }

    #[test]
    fn test_sort_hands(){
        let hand1 = Hand {
            hand_type: HandType::FiveOfKind,
            cards: vec![2,2,2,2,2],
            bid: 122,
        };
        let hand2 = Hand {
            hand_type: HandType::FiveOfKind,
            cards: vec![3,3,3,3,3],
            bid: 222,
        };
        let hand3 = Hand {
            hand_type: HandType::FourOfKind,
            cards: vec![4,3,3,3,3],
            bid: 222,
        };
        let hand4 = Hand {
            hand_type: HandType::HighCard,
            cards: vec![2,3,4,5,6],
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
        let converted_hand = vec![14,13,12,11,10,9,8,7,6,5,4,3,2];
        assert_eq!(convert_hand(hand, false), converted_hand);
        // Joker
        let joker_hand = "AKQT98765432J";
        let converted_joker_hand = vec![14,13,12,10,9,8,7,6,5,4,3,2,1];
        assert_eq!(convert_hand(joker_hand, true), converted_joker_hand);
    }

    #[test]
    fn test_hand_type(){
        // Standard
        let five_kind: Vec<u32> = vec![14,14,14,14,14];
        let four_kind: Vec<u32> = vec![14,14,14,14,5];
        let full_house: Vec<u32> = vec![14,14,14,9,9];
        let three_kind: Vec<u32> = vec![14,14,14,2,9];
        let two_pair: Vec<u32> = vec![2,2,3,3,4];
        let one_pair: Vec<u32> = vec![2,2,3,4,5];
        let high_card: Vec<u32> = vec![2,3,4,5,6];
        assert_eq!(derive_hand_type(&five_kind, false), HandType::FiveOfKind);
        assert_eq!(derive_hand_type(&four_kind, false), HandType::FourOfKind);
        assert_eq!(derive_hand_type(&full_house, false), HandType::FullHouse);
        assert_eq!(derive_hand_type(&three_kind, false), HandType::ThreeOfKind);
        assert_eq!(derive_hand_type(&two_pair, false), HandType::TwoPair);
        assert_eq!(derive_hand_type(&one_pair, false), HandType::OnePair);
        assert_eq!(derive_hand_type(&high_card, false), HandType::HighCard);
        // Joker
        let five_kind: Vec<u32> = vec![1,14,14,14,14];
        let four_kind: Vec<u32> = vec![1,14,14,14,5];
        let full_house: Vec<u32> = vec![1,14,14,9,9];
        let three_kind: Vec<u32> = vec![1,14,14,2,9];
        let two_pair: Vec<u32> = vec![1,2,3,3,4];
        let one_pair: Vec<u32> = vec![1,2,3,4,5];
        let high_card: Vec<u32> = vec![1,2,3,4,5];
        assert_eq!(derive_hand_type(&five_kind, false), HandType::FiveOfKind);
        assert_eq!(derive_hand_type(&four_kind, false), HandType::FourOfKind);
        assert_eq!(derive_hand_type(&full_house, false), HandType::FullHouse);
        assert_eq!(derive_hand_type(&three_kind, false), HandType::ThreeOfKind);
        assert_eq!(derive_hand_type(&two_pair, false), HandType::TwoPair);
        assert_eq!(derive_hand_type(&one_pair, false), HandType::OnePair);
        assert_eq!(derive_hand_type(&high_card, false), HandType::HighCard);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
