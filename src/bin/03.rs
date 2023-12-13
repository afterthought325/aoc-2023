advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let parts_catalog = lines
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut filtered_catalog = vec![vec![' '; parts_catalog[0].len()]; parts_catalog.len()];

    for (row, record) in parts_catalog.iter().enumerate() {
        for (column, cell) in record.iter().enumerate() {
            let mut val = ' ';
            if cell.is_numeric() {
                if is_part_number(&parts_catalog, row, column) {
                    val = *cell;
                }
            }
            filtered_catalog[row][column] = val;
        }
    }

    let mut total: u32 = 0;
    for mut record in filtered_catalog {
        record.dedup();
        let s = String::from_iter(record);
        let numbers: u32 = s
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .sum();
        total += numbers;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn is_part_number(catalog: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let start_row = if row > 0 { row - 1 } else { 0 };
    let end_row = if row == catalog.len() - 1 {
        row
    } else {
        row + 1
    };
    for slice in &catalog[start_row..end_row] {
        let start_col = if col > 0 { col - 1 } else { 0 };
        let end_col = if col == slice.len() { col } else { col + 1 };
        match slice[start_col..end_col]
            .iter()
            .any(|x| x.is_ascii_punctuation())
        {
            true => return true,
            false => continue,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
