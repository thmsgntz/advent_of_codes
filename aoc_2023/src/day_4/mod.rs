use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn run_day_4(file_name: &str) -> i32 {
    let lines = fs::read_to_string(format!("src/day_4/{}", file_name))
        .expect("Should have been able to read this");

    let mut sum = 0;
    for line in lines.split('\n') {
        if line.is_empty() {
            continue;
        }

        let vec_of_ints = extract_into_arrays(line).expect("Could not extract arrays");
        sum += count_winners(&vec_of_ints[0], &vec_of_ints[1]);
    }

    println!("{} sum: {}", file_name, sum);
    return sum;
}

pub(crate) fn run_day_4_bonus(file_name: &str, number_of_cards: u32) -> u32 {
    let lines = fs::read_to_string(format!("src/day_4/{}", file_name))
        .expect("Should have been able to read this");

    let mut hmap = generate_hash_map(number_of_cards);

    for (i, line) in lines.split('\n').enumerate() {
        if line.is_empty() {
            continue;
        }

        let vec_of_ints = extract_into_arrays(line).expect("Could not extract arrays");
        let card_number = (i + 1) as u32;
        let copies = count_winners_bonus(card_number, &vec_of_ints[0], &vec_of_ints[1]);
        let number_copies_of_current_i = get_number_of_copies(&hmap, &card_number);

        increment_hashmap(&mut hmap, copies.clone(), number_copies_of_current_i);
    }

    let sum = hmap.into_values().sum::<u32>();

    println!("{} sum: {}", file_name, sum);
    return sum;
}

fn get_number_of_copies(hmap: &HashMap<u32, u32>, card_number: &u32) -> u32 {
    *hmap.get(&card_number).expect("Gone too far in loop?")
}

fn generate_hash_map(max_card:u32) -> HashMap<u32, u32> {
    HashMap::from_iter((1..max_card+1).map(|i| (i, 1_u32)))
}

fn increment_hashmap(hmap: &mut HashMap<u32, u32>, copies: Vec<u32>, ntimes: u32) {
    if copies.is_empty() {
        return;
    }

    for copy in copies {
        hmap.entry(copy).and_modify(|num| *num += ntimes);
    }
}

fn extract_into_arrays(line: &str) -> Result<Vec<HashSet<i32>>, &'static str> {
    if let Some(index_colon) = line.find(':') {
        let s = &line[index_colon + 1..];
        let vec: Vec<HashSet<i32>> = s
            .split('|')
            .map(|s| {
                s.split_ascii_whitespace()
                    .map(|sub| return sub.parse::<i32>().unwrap())
                    .collect::<HashSet<i32>>()
            })
            .collect::<Vec<HashSet<i32>>>();
        return Ok(vec);
    }
    return Err("Could not find ':'");
}

fn count_winners(vec_of_winners: &HashSet<i32>, vec_you_have: &HashSet<i32>) -> i32 {
    let cpt = vec_of_winners.intersection(&vec_you_have).count();
    return if cpt > 0 {
        2_i32.pow((cpt - 1) as u32)
    } else {
        0
    };
}

fn count_winners_bonus(
    card_number: u32,
    vec_of_winners: &HashSet<i32>,
    vec_you_have: &HashSet<i32>,
) -> Vec<u32> {
    let cpt = vec_of_winners.intersection(&vec_you_have).count();

    if cpt > 0 {
        return (card_number + 1..card_number + 1 + cpt as u32).collect::<Vec<u32>>();
    }

    return vec![];
}

#[cfg(test)]
mod test {
    use super::{count_winners, count_winners_bonus, extract_into_arrays, run_day_4, generate_hash_map, increment_hashmap, run_day_4_bonus};
    use std::collections::HashSet;

    #[test]
    fn test_example() {
        assert_eq!(run_day_4("example.txt"), 13);
    }

    #[test]
    fn test_example_bonus() {
        assert_eq!(run_day_4_bonus("example.txt", 6), 30);
    }

    #[test]
    fn test_extract_into_arrays() {
        assert_eq!(
            extract_into_arrays("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            Ok(vec![
                HashSet::from([41, 48, 83, 86, 17]),
                HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])
            ])
        );
    }

    #[test]
    fn test_count_winners() {
        assert_eq!(
            count_winners(&HashSet::from([11, 12]), &HashSet::from([11])),
            1
        );
        assert_eq!(
            count_winners(&HashSet::from([11, 12]), &HashSet::from([13])),
            0
        );
        assert_eq!(
            count_winners(&HashSet::from([11, 12]), &HashSet::from([11, 12])),
            2
        );
        assert_eq!(
            count_winners(&HashSet::from([11, 12, 13]), &HashSet::from([11, 12, 13])),
            4
        );
        assert_eq!(
            count_winners(
                &HashSet::from([11, 12, 13, 14, 15]),
                &HashSet::from([11, 12, 13, 14, 15])
            ),
            16
        );
        assert_eq!(
            count_winners(&HashSet::from([]), &HashSet::from([11, 12, 13])),
            0
        );
        assert_eq!(
            count_winners(&HashSet::from([11, 12, 13]), &HashSet::from([])),
            0
        );
    }

    #[test]
    fn test_count_winners_bonus() {
        let to_test = vec![
            (3, vec![11, 12], vec![11], vec![4]),
            (5, vec![11, 12], vec![11, 12], vec![6, 7]),
            (6, vec![11, 12, 13], vec![11, 12, 13], vec![7, 8, 9]),
            (1, vec![11, 12], vec![14], vec![]),
            (1, vec![11, 12], vec![11], vec![2]),
        ];

        for (card_number, winners, you_have, expected) in to_test {
            assert_eq!(
                count_winners_bonus(
                    card_number,
                    &HashSet::from_iter(winners.into_iter()),
                    &HashSet::from_iter(you_have.into_iter())
                ),
                expected
            );
        }
    }

    #[test]
    fn test_hashmap() {
        let mut hmap = generate_hash_map(5);

        let mut keys: Vec<u32> = hmap.clone().into_keys().collect::<Vec<u32>>();
        keys.sort_unstable();
        assert_eq!(keys, vec![1,2,3,4,5]);

        let values: Vec<u32> = hmap.clone().into_values().collect::<Vec<u32>>();
        assert_eq!(values, vec![1, 1, 1, 1, 1]);

        increment_hashmap(&mut hmap, vec![4, 5], 1);

        assert_eq!(hmap.get(&4), Some(&2));
        assert_eq!(hmap.get(&5), Some(&2));
        assert_eq!(hmap.get(&1), Some(&1));

        increment_hashmap(&mut hmap, vec![2, 5], 3);

        assert_eq!(hmap.get(&4), Some(&2));
        assert_eq!(hmap.get(&5), Some(&5));
        assert_eq!(hmap.get(&2), Some(&4));
    }
}
