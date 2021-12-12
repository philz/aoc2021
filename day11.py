#!/usr/bin/python3


SAMPLE_INPUT = """
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"""

INPUT="""
4438624262
6263251864
2618812434
2134264565
1815131247
2612457325
8585767584
7217134556
2825456563
8248473584
"""

def neighbors(n):
    y = n // 10
    x = n % 10
    ret = []
    # row above:
    if y >= 1:
        ret.append( (y-1)*10 + x)
        if x >= 1:
            ret.append( (y-1)*10 + x - 1)
        if x < 9:
            ret.append( (y-1)*10 + x + 1)
    # same row
    if x >= 1:
        ret.append( (y)*10 + x - 1)
    if x < 9:
        ret.append( (y)*10 + x + 1)
    # next row
    if y < 9:
        ret.append( (y+1)*10 + x)
        if x >= 1:
            ret.append( (y+1)*10 + x - 1)
        if x < 9:
            ret.append( (y+1)*10 + x + 1)

    return ret

NEIGHBORS = [ neighbors(x) for x in range(0, 100) ]

#state = [ 0 for x in range(100) ]
#print_state()

number_chars = set([str(x) for x in range(10)])
state = map(int, [ x for x in INPUT if x in number_chars ])

def print_state():
    for x in range(10):
        print("".join(map(str, state[10*x:10*x+10])))


print_state()


flash_count = 0
for s in range(1, 10000000):
    print("step ", s)

    already_flashed = set()
    flashers = set()

    for i in range(100):
        state[i] += 1
        if state[i] > 9:
            flashers.add(i)

    while len(flashers) > 0:
        i = flashers.pop()
        already_flashed.add(i)
        for x in NEIGHBORS[i]:
            state[x] += 1
            if state[x] > 9:
                if x not in already_flashed:
                    flashers.add(x)
    
    for i in already_flashed:
        state[i] = 0
    flash_count += len(already_flashed)

    # print_state()

    if s == 101:
        print("part 1 solution", flash_count)

    if len(already_flashed) == 100:
        print("part 2 solution", s)
        break
