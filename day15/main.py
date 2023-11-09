import sys
import re

def main():
    if len(sys.argv) < 4:
        print("Give input file as arg1, y for part1 as arg2, maxy for part2 as arg3")
        return
    file_name = sys.argv[1]
    part1y = int(sys.argv[2])
    maxy = int(sys.argv[3])
    input = parse(file_name)
    covered = set()
    for sb in input:
        covered = covered.union(covered_in_row(sb, part1y))
    print("Part 1: " + str(len(covered)))
    for y in range(0, maxy+1):
        ranges = []
        for sb in input:
            x = covered_range(sb, y, maxy)
            if x == None:
                continue
            ranges.append(x)
        start, end = ranges.pop()
        last = len(ranges)
        while len(ranges) != 0:
            for i, r in enumerate(ranges):
                if r[0] <= start and end <= r[1]:
                    ranges.pop(i)
                    start = r[0]
                    end = r[1]
                elif start <= r[0] and r[1] <= end:
                    ranges.pop(i)
                elif r[0] <= start and start <= r[1] and r[1] <= end:
                    ranges.pop(i)
                    start = r[0]
                elif end <= r[1] and start <= r[0] and r[0] <= end:
                    ranges.pop(i)
                    end = r[1]
            if last == len(ranges):
                print(y, start, end, ranges)
                return
            last = len(ranges)

def manhattan_disance(sb: tuple[int, int, int, int]) -> int:
    [sx, sy, bx, by] = sb
    return abs(sx - bx) + abs(sy - by) 

def covered_in_row(sb: tuple[int, int, int, int], y: int) -> set[int]:
    covered = set()
    r = manhattan_disance(sb)
    [sx, sy, bx, by] = sb
    if (y > sy + r) or (y < sy - r):
        return covered
    diff = r - abs(sy-y)
    start = sx-diff
    end = sx+diff
    for i in range(start, end+1):
        covered.add(i)
    if by == y:
        covered.remove(bx)
    if sy == y:
        covered.remove(sx)
    return covered

def covered_range(sb: tuple[int, int, int, int], y: int, maxx: int) -> tuple[int, int] | None:
    r = manhattan_disance(sb)
    [sx, sy, bx, by] = sb
    if (y > sy + r) or (y < sy - r):
        return None
    diff = r - abs(sy-y)
    start = sx-diff
    end = sx+diff
    return max(0, start), min(end, maxx)

def parse(file_name: str) -> set[tuple[int, int, int, int]]:
    pattern = re.compile("\D+x=(-?\d+), y=(-?\d+)\D+x=(-?\d+), y=(-?\d+)")
    input = set()
    with open(file_name) as f:
        for line in f:
            res = list(map(int, re.search(pattern, line).groups()))
            input.add((res[0], res[1], res[2], res[3]))
    return input

if __name__ == "__main__":
    main()
