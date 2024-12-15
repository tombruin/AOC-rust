advent_of_code::solution!(5);

use std::{cmp::Ordering, collections::HashMap};

pub fn part_one(input: &str) -> Option<u32> {
    let (input_ordering, input_updates) = input.split_once("\n\n").unwrap();
    let mut rules_map: HashMap<_, Vec<_>> = input_ordering.lines().fold(HashMap::new(), |mut ordering_rules, rule| {
        let mut splitted_rules = rule.split('|');
        ordering_rules
            .entry(splitted_rules.next().unwrap().parse::<usize>().unwrap())
            .or_default()
            .push(splitted_rules.next().unwrap().parse::<usize>().unwrap());
        ordering_rules
    });

    //Sort the page before numbers
    rules_map.values_mut().for_each(|page_before| page_before.sort_unstable());
    // println!("rules_map: {:?}", rules_map);

    let answer = input_updates.lines().map(|updates| {
        updates
            .split(',')
            .map(|update| update.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    }).filter(|printer_input| {
        for (i, page) in printer_input.iter().enumerate() {
            //Check if rule found for this page number
            if let Some(rules_for_page) = rules_map.get(page) {
                //Check all printer input pages in the rules.
                if printer_input[0..i]
                    .iter()
                    .any(|&page| rules_for_page.binary_search(&page).is_ok())
                {
                    return false;
                }
            }
        }
        return true
    }).map(|pages| pages[pages.len() / 2])
    .sum::<usize>();

    // println!("Answer: {:?}", answer);
    Some(answer.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (input_ordering, input_updates) = input.split_once("\n\n").unwrap();
    let mut rules_map: HashMap<_, Vec<_>> = input_ordering.lines().fold(HashMap::new(), |mut ordering_rules, rule| {
        let mut splitted_rules = rule.split('|');
        ordering_rules
            .entry(splitted_rules.next().unwrap().parse::<usize>().unwrap())
            .or_default()
            .push(splitted_rules.next().unwrap().parse::<usize>().unwrap());
        ordering_rules
    });

    //Sort the page before numbers
    rules_map.values_mut().for_each(|page_before| page_before.sort_unstable());
    // println!("rules_map: {:?}", rules_map);

    let answer = input_updates.lines().map(|updates| {
        updates
            .split(',')
            .map(|update| update.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    }).filter(|printer_input| {
        for (i, page) in printer_input.iter().enumerate() {
            //Check if rule found for this page number
            if let Some(rules_for_page) = rules_map.get(page) {
                //Check all print input pages Incorrectly in the rules
                if printer_input[0..i]
                    .iter()
                    .any(|&page| rules_for_page.binary_search(&page).is_ok())
                {
                    return true;
                }
            }
        }
        return false
    }).map(|mut pages| {
        // println!("Wrong printer input: {:?}", pages);
        pages.sort_by(|a, b| {
            if rules_map
                .get(a)
                .is_some_and(|rules_map| rules_map.binary_search(&b).is_ok())
            {
                Ordering::Less // A before B
            } else {
                Ordering::Greater // A after B
            }
        });
        pages[pages.len() / 2]
    })
    .sum::<usize>();

    // println!("Answer: {:?}", answer);
    Some(answer.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
