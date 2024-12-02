advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let data: Vec<Vec<i32>> = lines
        .iter()
        .map(|x| {
            let x = x
                .split_ascii_whitespace()
                .map(|y| y.trim().parse::<i32>().unwrap());
             x.collect()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut answer: u32 = 0;
    for report in data {
        let difference_valid: bool = report.windows(2).all(|w| w[0].abs_diff(w[1]) <= 3 );
        let decreasing: bool = report.windows(2).all(|w| w[0] > w[1] );
        let increasing: bool = report.windows(2).all(|w| w[0] < w[1] );
        let total_valid: bool = (decreasing || increasing) && difference_valid;

        // println!("total_valid: {:?}", total_valid);
        answer += u32::from(total_valid);
    }
    // println!("total_valid: {:?}", answer);
    return Some(answer);
}


pub fn is_valid(input: &Vec<i32>) -> bool {
    let difference_valid: bool = input.windows(2).all(|w| w[0].abs_diff(w[1]) <= 3 );
    let decreasing: bool = input.windows(2).all(|w| w[0] > w[1] );
    let increasing: bool = input.windows(2).all(|w| w[0] < w[1] );
    return (decreasing || increasing) && difference_valid;
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let data: Vec<Vec<i32>> = lines
        .iter()
        .map(|x| {
            let x = x
                .split_ascii_whitespace()
                .map(|y| y.trim().parse::<i32>().unwrap());
             x.collect()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut answer: u32 = 0;
    for report in data {
        let mut total_valid: bool = false;
        for i in 0 .. report.len()
        {
            total_valid = is_valid(&report);
            if total_valid == false {
                let mut temp_report = report.clone();
                temp_report.remove(i);
                total_valid  = is_valid(&temp_report);
                if total_valid == true {
                    break;
                }
            }
            else {
                break;
            }
        }
        answer += u32::from(total_valid);
    }
    return Some(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
