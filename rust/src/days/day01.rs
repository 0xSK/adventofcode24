use std::iter::zip; 

pub fn solve () {
    let input = include_str!("../../../inputs/01.txt");
    
    let mut vec1: Vec<i64> = Vec::new();
    let mut vec2: Vec<i64> = Vec::new();

    for line in input.lines() {
        let line_numbers: Vec<i64> = line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();

        vec1.push(line_numbers[0]);
        vec2.push(line_numbers[1]);
    }

    vec1.sort();
    vec2.sort();
    
    // part 1
    let part1_answer = zip(vec1.clone(), vec2.clone()).map(|(a, b)| i64::abs(a - b)).sum::<i64>();
    println!("Part1: {}", part1_answer);

    // part 2
    let mut vec2_counts: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();
    for elem in vec2 {
        *vec2_counts.entry(elem).or_insert(0) += 1;
    }

    let vec1_elems_counts_in_vec2: Vec<i64> = vec1.iter().map(|elem| *vec2_counts.get(elem).unwrap_or(&0)).collect();

    let part2_answer = zip(vec1.clone(), vec1_elems_counts_in_vec2).map(|(a, b)| a * b).sum::<i64>();

    println!("Part2: {}", part2_answer);


}