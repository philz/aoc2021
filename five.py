#!/usr/bin/env python3
import re

def part1():
    counts = []
    for i in range(0, 1000):
        counts.append([0]*1000)

    def p():
        return
        print("")
        for i in range(0, 11):
            print("".join(map(str, counts[i][0:11])))

    for line in open("input5.txt"):
        a,b,c,d = [ int(x) for x in re.split("[^0-9]", line) if x != "" ]
        if a == c:
            b1 = min(b,d)
            b2 = max(b,d)
            for y in range(b1, b2 + 1):
                counts[y][a] += 1
            p()
        if b == d:
            a1 = min(a,c)
            a2 = max(a,c)
            for y in range(a1, a2 + 1):
                counts[b][y] += 1
            p()

    ret = 0
    for i in range(0, 1000):
        for j in range(0, 1000):
            if counts[i][j] >= 2:
                ret += 1

    print(ret)


def part2():
    counts = []
    for i in range(0, 1000):
        counts.append([0]*1000)

    def p():
        print("")
        for i in range(0, 11):
            print("".join(map(str, counts[i][0:11])))

    for line in open("input5.txt"):
        a,b,c,d = [ int(x) for x in re.split("[^0-9]", line) if x != "" ]
        steps = max( abs(c-a), abs(d-b) )
        if a == c:
            xstep = 0
        elif a < c:
            xstep = 1
        else:
            xstep = -1

        if b == d:
            ystep = 0
        elif b < d:
            ystep = 1
        else:
            ystep = -1

        x = a
        y = b

        print(line)
        for _ in range(steps + 1):
            print(y, x)
            counts[y][x] += 1
            x += xstep
            y += ystep
        p()

    ret = 0
    for i in range(0, 1000):
        for j in range(0, 1000):
            if counts[i][j] >= 2:
                ret += 1

    print(ret)


part1()
part2()
