use std::collections::VecDeque;

use regex::Regex;

pub fn solve () {
    let input: String = std::fs::read_to_string("../inputs/03.txt").unwrap();

    // part 1
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let part1_mac: i64 = re.captures_iter(&input).map(|cap| {
        let x: i64 = cap[1].parse::<i64>().unwrap();
        let y: i64 = cap[2].parse::<i64>().unwrap();
        return x * y;
    }).sum();

    println!("Part 1: {part1_mac}");

    // part 2, similar to the python solution:

    let mut do_positions: VecDeque<usize> = Regex::new(r"do\(\)").unwrap().captures_iter(&input).map(|cap| cap.get(0).unwrap().start()).collect();
    let mut dont_positions: VecDeque<usize> = Regex::new(r"don't\(\)").unwrap().captures_iter(&input).map(|cap| cap.get(0).unwrap().start()).collect();

    let mut disabled_sections: VecDeque<(usize, usize)> = VecDeque::new();

    let mut curr_section_enabled: bool = true;
    let mut last_disabled_start_position: usize = 0;

    // now we iterate over the `do` and `don't` positions to find the disabled sections

    while do_positions.len() > 0 && dont_positions.len() > 0 {
        if do_positions[0] < dont_positions[0] {
            let next_position = do_positions.pop_front().unwrap();
            if !curr_section_enabled {
                disabled_sections.push_back((last_disabled_start_position, next_position));
            }
            curr_section_enabled = true;
        } else {
            let next_position = dont_positions.pop_front().unwrap();
            if curr_section_enabled {
                last_disabled_start_position = next_position;
            }
            curr_section_enabled = false;
        }
    }
    
    // boundary case
    if dont_positions.len() > 0 {
        if !curr_section_enabled {
            disabled_sections.push_back((last_disabled_start_position, input.len()));
        } else {
            last_disabled_start_position = dont_positions.pop_front().unwrap();
            disabled_sections.push_back((last_disabled_start_position, input.len()));
        }
    }
    
    // get all the muls, filter out the disabled sections
    let part2_mac: i64 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap().captures_iter(&input).filter(|cap| {
        let mul_position = cap.get(0).unwrap().start();

        while disabled_sections.len() > 0 && disabled_sections[0].1 < mul_position {
            disabled_sections.pop_front();
        }

        if disabled_sections.len() > 0 && disabled_sections[0].0 <= mul_position && disabled_sections[0].1 > mul_position {
            return false;
        } else {
            return true;
        }
    }).map(|cap| {
        let x: i64 = cap[1].parse::<i64>().unwrap();
        let y: i64 = cap[2].parse::<i64>().unwrap();
        return x * y;
    }).sum();

    println!("Part 2: {part2_mac}");

}