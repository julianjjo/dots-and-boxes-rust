
use std::collections::HashSet;

pub fn calulate_score(grid: &Vec<Vec<Vec<Vec<i8>>>>, player_1: &mut i32, player_2: &mut i32) {
    for row in grid.iter() {
        for column in row.iter() {
            let amount_player_lines = count_unique_numbers(&column[1]);
            if amount_player_lines == 1 && column[1][0] != 0 {
                if column[1][0] == 1 {
                    *player_1 += 1;
                } else {
                    *player_2 += 1;
                }
            }
        }
    }
}

pub fn is_valid_position<T>(index: usize, vector: &Vec<T>) -> bool {
    index < vector.len()
}

pub fn count_unique_numbers(vector: &Vec<i8>) -> usize {
    let mut unique_set = HashSet::new();
    for &num in vector {
        unique_set.insert(num);
    }
    unique_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_unique_numbers_test() {
        let vector: Vec<i8> = vec![1, 1, 1, 1];
        assert_eq!(count_unique_numbers(&vector), 1);
    }

    #[test]
    fn count_unique_numbers_two_test() {
        let vector: Vec<i8> = vec![1, 1, 0, 0];
        assert_eq!(count_unique_numbers(&vector), 2);
    }

    #[test]
    fn is_valid_position_test() {
        let vector: Vec<i8> = vec![1, 1, 1, 1];
        assert_eq!(is_valid_position(1, &vector), true);
    }
}