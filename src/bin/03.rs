advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((?P<num1>\d+),(?P<num2>\d+)\)").unwrap();
    let answer: u32 = regex.captures_iter(input).map(|caps| {

        let number1: u32 = caps.name("num1").unwrap().as_str().parse::<u32>().unwrap();
        let number2: u32 = caps.name("num2").unwrap().as_str().parse::<u32>().unwrap();
        number1 * number2
    }).sum();
    //println!("Answer: {:?}", answer);
    return Some(answer);
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((?P<num1>\d+),(?P<num2>\d+)\)|(?P<do>do\(\))|(?P<dont>don['’]t\(\))|
").unwrap();

    let mut enable_multification: bool = true;
    let answer: u32 = regex.captures_iter(input).map(|caps| {
        let mut number1: u32 = 0;
        let mut total: u32 = 0;
        if caps.name("num1") != None {
            number1 = caps.name("num1").unwrap().as_str().parse::<u32>().unwrap();
        }
        if caps.name("num2") != None {
            let number2: u32 = caps.name("num2").unwrap().as_str().parse::<u32>().unwrap();
            if enable_multification {
                total = number1 * number2;
            }
        }
        if caps.name("do") != None {
            enable_multification = true;
        }
        if caps.name("dont") != None {
            enable_multification = false;
        }
        total
    }).sum();

    //println!("Answer: {:?}", answer);
    return Some(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
