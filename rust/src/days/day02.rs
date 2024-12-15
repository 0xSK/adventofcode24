pub fn solve () {
    let input = include_str!("../../../inputs/02.txt");
    
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let report: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        reports.push(report);
    }

    // part 1

    // # rule a:
    // all levels in the report are strictly increasing or strictly decreasing
    let rule1a = |report: &Vec<i32>| -> bool {
        report.windows(2).all(|pair| pair[0] < pair[1]) || report.windows(2).all(|pair| pair[0] > pair[1])
    };

    // # rule b:
    // two adjacent levels differ by at least one and at most three
    let rule1b = |report: &Vec<i32>| -> bool {
        report.windows(2).all(|pair| (pair[1] - pair[0]).abs() <= 3 && (pair[1] - pair[0]).abs() >= 1)
    };

    let num_safe_reports_part1: i32 = reports.iter().filter(|report| rule1a(report) && rule1b(report)).count() as i32;
    
    println!("Part 1: {num_safe_reports_part1}");

    // part 2

    // I know there's a better way to do this using dynamic programming
    // but I am catching up on advent of code and don't have a lot of time left
    // so here's a naive brute force solution
    // FIXME: come back and improve this implementation if you have time

    let mut num_safe_reports_part2: i32 = 0;

    for report in reports {
        let mut report_copies_to_check: Vec<Vec<i32>> = vec![report.clone()];

        for i in 0..report.len() {
            let mut report_copy = report.clone();
            report_copy.remove(i);
            report_copies_to_check.push(report_copy);
        }

        if report_copies_to_check.iter().any(|report_copy| rule1a(report_copy) && rule1b(report_copy)) {
            num_safe_reports_part2 += 1;
        }
    }

    println!("Part 2: {num_safe_reports_part2}");
}