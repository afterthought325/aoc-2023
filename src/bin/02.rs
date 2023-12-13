advent_of_code::solution!(2);

#[derive(Debug)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut games = Vec::<Vec<Draw>>::new();

    for line in lines {
        let game: Vec<_> = line.split(":").last().unwrap().split(";").collect();
        let mut draws = Vec::<Draw>::new();
        for x in &game {
            let mut draw = Draw {
                red: 0,
                green: 0,
                blue: 0,
            };
            let colors: Vec<_> = x.split(",").collect();
            for color in colors {
                if color.contains("red") {
                    let red = color.trim().split(" ").next().unwrap();
                    draw.red = red.parse().unwrap();
                } else if color.contains("green") {
                    let green = color.trim().split(" ").next().unwrap();
                    draw.green = green.parse().unwrap();
                } else if color.contains("blue") {
                    let blue = color.trim().split(" ").next().unwrap();
                    draw.blue = blue.parse().unwrap();
                }
            }
            draws.push(draw);
        }
        games.push(draws);
    }

    let mut total = 0;
    for (position, game) in games.iter().enumerate() {
        if check_game(game) {
            total += position + 1;
        }
    }

    Some(u32::try_from(total).unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut games = Vec::<Vec<Draw>>::new();

    for line in lines {
        let game: Vec<_> = line.split(":").last().unwrap().split(";").collect();
        let mut draws = Vec::<Draw>::new();
        for x in &game {
            let mut draw = Draw {
                red: 0,
                green: 0,
                blue: 0,
            };
            let colors: Vec<_> = x.split(",").collect();
            for color in colors {
                if color.contains("red") {
                    let red = color.trim().split(" ").next().unwrap();
                    draw.red = red.parse().unwrap();
                } else if color.contains("green") {
                    let green = color.trim().split(" ").next().unwrap();
                    draw.green = green.parse().unwrap();
                } else if color.contains("blue") {
                    let blue = color.trim().split(" ").next().unwrap();
                    draw.blue = blue.parse().unwrap();
                }
            }
            draws.push(draw);
        }
        games.push(draws);
    }

    let mut total = 0;
    for game in games {
        let (red, green, blue) = find_min_dice(&game);
        let power = red * green * blue;
        total += power;
    }
    Some(u32::try_from(total).unwrap())
}

fn check_game(game: &Vec<Draw>) -> bool {
    for draw in game {
        if draw.red > MAX_RED {
            //println!("red: {} is greater than max: {}", draw.red, MAX_RED);
            return false;
        } else if draw.green > MAX_GREEN {
            //println!("green: {} is greater than max: {}", draw.green, MAX_GREEN);
            return false;
        } else if draw.blue > MAX_BLUE {
            //println!("blue: {} is greater than max: {}", draw.blue, MAX_BLUE);
            return false;
        }
    }
    true
}

fn find_min_dice(game: &Vec<Draw>) -> (usize, usize, usize) {
    let mut red = game.iter().map(|x| x.red).max().unwrap();
    let mut green = game.iter().map(|x| x.green).max().unwrap();
    let mut blue = game.iter().map(|x| x.blue).max().unwrap();

    if red == 0 {
        red = 1
    };
    if green == 0 {
        green = 1
    };
    if blue == 0 {
        blue = 1
    };

    (red, green, blue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
