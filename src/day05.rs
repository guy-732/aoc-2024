use fnv::{FnvHashMap, FnvHashSet};

#[aoc_generator(day05)]
fn parse(input: &str) -> (FnvHashMap<u64, FnvHashSet<u64>>, Vec<Vec<u64>>) {
    let mut map = FnvHashMap::default();
    let mut lines = input.lines();
    for line in lines.by_ref().take_while(|&line| !line.is_empty()) {
        let (left, right) = line.split_once('|').expect("Expected '|' in mapping");
        let entry = map
            .entry(right.parse().expect("Failed to parse int"))
            .or_insert_with(FnvHashSet::default);
        entry.insert(left.parse().expect("Failed to parse int"));
    }

    (
        map,
        lines
            .map(|line| {
                line.split(',')
                    .map(|num| num.parse().expect("Failed to parse int"))
                    .collect()
            })
            .collect(),
    )
}

fn is_valid_print_order(order: &[u64], mapping: &FnvHashMap<u64, FnvHashSet<u64>>) -> bool {
    for (index, &page) in order.iter().enumerate() {
        if let Some(required) = mapping.get(&page) {
            for required in required {
                if order[index..].contains(required) && !order[..index].contains(required) {
                    return false;
                }
            }
        }
    }

    true
}

#[aoc(day05, part1)]
fn part1(input: &(FnvHashMap<u64, FnvHashSet<u64>>, Vec<Vec<u64>>)) -> u64 {
    let (mapping, orders) = input;

    orders
        .iter()
        .filter(|&order| is_valid_print_order(order, mapping))
        // .inspect(|order| println!("{order:?}"))
        .map(|order| order[order.len() / 2])
        // .inspect(|value| println!("{value}"))
        .sum()
}

fn insert_page(
    reordered: &mut Vec<u64>,
    page: u64,
    rest_of_order: &[u64],
    mapping: &FnvHashMap<u64, FnvHashSet<u64>>,
) {
    if reordered.contains(&page) {
        return;
    }

    if let Some(required) = mapping.get(&page) {
        for &required in required {
            if rest_of_order.contains(&required) && !reordered.contains(&required) {
                insert_page(reordered, required, rest_of_order, mapping);
            }
        }
    }

    reordered.push(page);
}

fn correctly_reorder_and_get_middle(
    order: &[u64],
    mapping: &FnvHashMap<u64, FnvHashSet<u64>>,
) -> u64 {
    let mut reordered = vec![];

    for (i, &page) in order.iter().enumerate() {
        insert_page(&mut reordered, page, &order[i..], mapping);
    }

    // println!("Reordered: {reordered:?}");
    reordered[reordered.len() / 2]
}

#[aoc(day05, part2)]
fn part2(input: &(FnvHashMap<u64, FnvHashSet<u64>>, Vec<Vec<u64>>)) -> u64 {
    let (mapping, orders) = input;

    orders
        .iter()
        .filter(|&order| !is_valid_print_order(order, mapping))
        // .inspect(|order| println!("{order:?}"))
        .map(|order| correctly_reorder_and_get_middle(order, mapping))
        // .inspect(|value| println!("{value}"))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 123);
    }
}
