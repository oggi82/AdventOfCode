
def main():
    # part 1
    lines = [int(a) for a in open("input.txt").readlines()]
    print(sum((1 if b > a else 0) for a, b in zip(lines[:-1], lines[1:])))
    # part 2
    l = [lines[i:i+3] for i in range(0, len(lines)-2)]
    print(sum((1 if sum(b) > sum(a) else 0) for a, b in zip(l[:-1], l[1:])))


if __name__ == '__main__':
    main()
