"""
Interval Script to solve the following problem:

Given a series of intervals which can be either individual numbers, a range of numbers with a step size, or an
infinite range of numbers with a step size.
Find how many numbers are not included in any of these intervals.
"""


val1 = [i for i in range(0, 101, 5)]
val2 = [2]
val3 = [i for i in range(3, 100, 3)]
val4 = [i for i in range(1, 1002, 10)]


def brute_solution(start: int, end: int, *intervals: list[int]):
    """
    Finds our solution by brute force checking whether a number is stepped on.
    :param start: the starting number
    :param end: the ending number (inclusive)
    :param intervals: the intervals we want to avoid.
    :return: How many numbers are not touched by the intervals.
    """
    current = start
    steppedon = 0
    touched = []
    while current <= end:
        for i in intervals:
            if current in i:
                steppedon += 1
                touched.append(current)
                break
        current += 1

    return end - start - steppedon, touched


def smart_solution(start: int, end: int, *intervals: list[int]):
    # Iterate over the intervals, checking their steps, and removing collisions.
    pass

result, touched = brute_solution(0, 500, val1, val2, val3, val4)

print(result)
print(len(touched))
print(touched)
