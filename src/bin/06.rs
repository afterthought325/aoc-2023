use std::iter::zip;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let lines: Vec<_> = input.split('\n').collect();
    let times: Vec<usize> = lines[0]
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let distances: Vec<usize> = lines[1]
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    //let races = zip(times, distances).map(|x| Race { time: x.0, distance: x.1}).collect();
    let races: Vec<(usize, usize)> = zip(times, distances).collect();
    let mut answer = 1;
    for (time, distance) in races {
        let min = (0..time + 1)
            .map(|x| dist(x, time))
            .position(|x| x > distance)
            .unwrap();
        let max = (0..time + 1)
            .map(|x| dist(x, time))
            .rposition(|x| x > distance)
            .unwrap();
        let solutions = max - min + 1;
        answer *= solutions;
    }

    Some(answer)
}

fn dist(x: usize, time: usize) -> usize {
    x * (time - x)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines: Vec<_> = input.split('\n').collect();
    let time: usize = lines[0]
        .strip_prefix("Time:")
        .unwrap()
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();
    let distance: usize = lines[1]
        .strip_prefix("Distance:")
        .unwrap()
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();
    //let races = zip(times, distances).map(|x| Race { time: x.0, distance: x.1}).collect();
    let min = (0..time + 1)
        .map(|x| dist(x, time))
        .position(|x| x > distance)
        .unwrap();
    let max = (0..time + 1)
        .map(|x| dist(x, time))
        .rposition(|x| x > distance)
        .unwrap();
    let answer = max - min + 1;

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
