#!/usr/bin/env python3.13

import numpy as np
import pandas as pd
from collections import Counter

if __name__ == "__main__":
    with open("../inputs/01.txt") as inputFile:
        array = np.genfromtxt(inputFile, dtype=int, autostrip=True)
    
    list1 = np.sort(array[:,0])
    list2 = np.sort(array[:,1])

    # part1
    part1Answer = np.sum(np.abs(list1 - list2))
    print(f"Part 1: {part1Answer}")
    

    # part2
    list2Counts = Counter(list2)
    list1ElemsCountsInList2 = np.array(list(map(list2Counts.__getitem__, list1)))

    part2Answer = np.dot(list1, list1ElemsCountsInList2)
    print(f"Part 2: {part2Answer}")
