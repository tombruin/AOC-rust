// use std::collections::HashMap;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {

    const DIRECTIONS: [(isize, isize); 8] = [
        (1, -1),  // NE
        (1, 0),   // E
        (1, 1),   // SE
        (0, -1),  // N
        (0, 1),  // S
        (-1, -1), // NW
        (-1, 0), // W
        (-1, 1), // SW
    ];

    let mut word = [0; 4];
    let data: Vec<_> = input.lines().map(|s| s.as_bytes()).collect();
    let answer = (0..data[0].len() as isize)
    .flat_map(|x| (0..data.len() as isize).map(move |y| (x, y)))
    .flat_map(|(x, y)| {
        DIRECTIONS.iter().map(move | &(dx, dy) | {
            vec![(x, y),
                (x + dx * 1, y + dy * 1),
                (x + dx * 2, y + dy * 2),
                (x + dx * 3, y + dy * 3)
            ]
        })
    })
    .filter(|coords| {
        let mut iter = coords.iter().map(|(x, y)| {
            data.get(*y as usize)
                .and_then(|row| row.get(*x as usize).copied())
                .unwrap_or_default()
        });
        word.fill_with(|| iter.next().unwrap_or_default());
        //println!("word: {:?}", String::from_utf8(word.to_vec()));
        &word == b"XMAS"
    }).count();

    //println!("Answer: {:?}", answer);
    Some(answer.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut cross = [0; 6];
    let data: Vec<_> = input.lines().map(|s| s.as_bytes()).collect();
    let answer = (0..data[0].len() as isize)
    .flat_map(|x| (0..data.len() as isize).map(move |y| (x, y)))
    .map(|(x, y)| {
        [
            (x + 2, y),     // NW
            (x + 1, y + 1), // Center
            (x, y + 2),     // SE
            (x, y),         // NE
            (x + 1, y + 1), // Center
            (x + 2, y + 2), // SW
        ]
    })
    .filter(|coords| {
        let mut iter = coords.iter().map(|(x, y)| {
            data.get(*y as usize)
                .and_then(|row| row.get(*x as usize).copied())
                .unwrap_or_default()
        });

        cross[0] = iter.next().unwrap_or_default();
        cross[1] = iter.next().unwrap_or_default();
        cross[2] = iter.next().unwrap_or_default();
        cross[3] = iter.next().unwrap_or_default();
        cross[4] = iter.next().unwrap_or_default();
        cross[5] = iter.next().unwrap_or_default();

        let testmiddle = cross[1];
        if testmiddle != b'A' {
            return false;
        }
        //println!("word: {:?}", String::from_utf8(cross.to_vec()));
        &cross == b"MASMAS" || &cross == b"SAMSAM" || &cross == b"SAMMAS" || &cross == b"MASSAM"
    })
    .count();
    //println!("Answer: {:?}", answer);
    Some(answer.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
