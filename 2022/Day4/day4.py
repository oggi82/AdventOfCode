

def readfile():
    content = []
    with open('input.txt') as file:
        content = file.readlines()

    c = [x.strip().split(',') for x in content]
    return c

def part1():
    f = readfile()
    ctr = 0
    for i in f:
        a, b = map(int, i[0].split('-'))
        c, d = map(int, i[1].split('-'))
        set_a = set(range(a, b+1))
        set_b = set(range(c, d+1))
        if set_a.issubset(set_b) or set_b.issubset(set_a):
            ctr += 1
    return ctr

def part2():
    f = readfile()
    ctr = 0
    for i in f:
        a, b = map(int, i[0].split('-'))
        c, d = map(int, i[1].split('-'))
        set_a = set(range(a, b+1))
        set_b = set(range(c, d+1))
        if not set_a.isdisjoint(set_b):
            ctr += 1
    return ctr

def main():
    ctr = part1()
    print('{} sections are subset from another section'.format(ctr))
    ctr = part2()
    print('{} sections have overlap ranges'.format(ctr))
    

if __name__ == '__main__':
    main()
