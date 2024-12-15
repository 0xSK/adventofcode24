#!/usr/bin/env python3.13

import numpy as np
import pandas as pd
from itertools import pairwise

if __name__ == "__main__":
    inputLines = open("../inputs/02.txt").read().splitlines()
    reports: list[list[int]] = [list(map(int, line.split())) for line in inputLines]

    # part1
    numSafeReports: int = 0


    # rule a:
    # all levels in the report are strictly increasing or strictly decreasing
    rule1a = lambda report: all(a < b for a, b in pairwise(report)) or all(a > b for a, b in pairwise(report))

    # rule b:
    # two adjacent levels differ by at least one and at most three
    rule1b = lambda report: all(1 <= abs(a - b) <= 3 for a, b in pairwise(report))

    numSafeReports = sum(rule1a(report) and rule1b(report) for report in reports)

    print(f"Part 1: {numSafeReports}")

    # part2
    # I know there's a better way to do this using dynamic programming
    # but I am catching up on advent of code and don't have a lot of time left
    # so here's a naive brute force solution
    # FIXME: come back and improve this implementation if you have time

    numSafeReports = 0

    for report in reports:
        reportCopiesToCheck: list[list[int]] = [report]
        # add dampened copies of the report
        # in each dampened copy, one element is removed
        for i in range(len(report)):
            dampenedReport = report.copy()
            dampenedReport.pop(i)
            reportCopiesToCheck.append(dampenedReport)

        numSafeReports += int(any(rule1a(potentialReport) and rule1b(potentialReport) for potentialReport in reportCopiesToCheck))
    
    print(f"Part 2: {numSafeReports}")
