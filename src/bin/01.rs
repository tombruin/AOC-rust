advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();
    for line in input.lines() {
        let mut splitted_line = line.split_whitespace();
        first_list.push(splitted_line.next()?.parse::<u32>().unwrap());
        second_list.push(splitted_line.next()?.parse::<u32>().unwrap());
    }

    first_list.sort();
    second_list.sort();

    let mut difference: u32 = 0;
    first_list
        .iter()
        .zip(&second_list)
        .for_each(|f: (&u32, &u32)| difference += f.0.abs_diff(*f.1));
    return Some(difference);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();
    for line in input.lines() {
        let mut splitted_line = line.split_whitespace();
        first_list.push(splitted_line.next()?.parse::<u32>().unwrap());
        second_list.push(splitted_line.next()?.parse::<u32>().unwrap());
    }

    let mut difference: u32 = 0;
    for location_id in first_list {
        let location_id_count = second_list.iter().filter(|&location| *location == location_id).count() as u32;
        difference += location_id_count * location_id;
    }

    // println!("difference: {}", difference);
    return Some(difference);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
