#!/usr/bin/env python3.13

import re
import numpy as np

if __name__ == "__main__":
    input = open("../inputs/03.txt").read()

    # part1
    # find all the valid `mul(X,Y)` substrings in the input
    validMulsFromInput: list[tuple[str, str]] = re.findall(r"mul\((\d{1,3}),(\d{1,3})\)", input)
    
    # convert the strings found into integers, also put them in a numpy array
    inputArray: np.ndarray = np.array([[int(t[0]), int(t[1])] for t in validMulsFromInput])
    
    # multiple all pairs together and sum them together for the final answer
    part1Mac: int = np.dot(inputArray[:,0], inputArray[:,1])

    print(f"Part 1: {part1Mac}")

    # part2
    doPositions: list[int] = [0] + [f.start() for f in re.finditer(r"do\(\)", input)]
    dontPositions: list[int] = [f.start() for f in re.finditer(r"don't\(\)", input)]

    disabledSections: list[tuple[int, int]] = []

    # now we iterate over the `do` and `don't` positions to find the disabled sections
    currSectionEnabled: bool = True
    lastDisabledStartPosition: int = 0

    while doPositions and dontPositions:
        if doPositions[0] < dontPositions[0]:
            nextPosition: int = doPositions.pop(0)
            if not currSectionEnabled:
                disabledSections.append((lastDisabledStartPosition, nextPosition))
            currSectionEnabled = True
        else:
            nextPosition: int = dontPositions.pop(0)
            if currSectionEnabled:
                lastDisabledStartPosition = nextPosition
            currSectionEnabled = False
    # boundary case
    if dontPositions:
        if not currSectionEnabled:
            disabledSections.append((lastDisabledStartPosition, len(input)))
        else:
            lastDisabledStartPosition = dontPositions.pop(0)
            disabledSections.append((lastDisabledStartPosition, len(input)))

    # get all the muls, and filter out the disabled sections
    part2Mac: int = 0
    for mulMatch in re.finditer(r"mul\((\d{1,3}),(\d{1,3})\)", input):
        mulPosition: int = mulMatch.start()
        # we linearly scan through the disabled sections, discarding the disabled sections that start before the mul position
        while disabledSections and disabledSections[0][1] < mulPosition:
            disabledSections.pop(0)
        # check if the mul position is in a disabled section
        if disabledSections and disabledSections[0][0] <= mulPosition < disabledSections[0][1]:
            continue
        else:
            part2Mac += int(mulMatch.group(1)) * int(mulMatch.group(2))

    print(f"Part 2: {part2Mac}")
