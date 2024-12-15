def main():
    lines = []
    with open("input.txt") as f:
        lines = f.readlines()
    horz = 0
    vert = 0
    # --- part one ---
    for a in lines:
        s = a.split(' ')
        if s[0] == 'forward':
            horz += int(s[1])
        elif s[0] == 'down':
            vert += int(s[1])
        elif s[0] == 'up':
            vert -= int(s[1])
    print(horz * vert)
    # --- part two ---
    horz = 0
    vert = 0
    aim = 0
    for a in lines:
        s = a.split(' ')
        if s[0] == 'forward':
            horz += int(s[1])
            vert += int(s[1]) * aim
        elif s[0] == 'down':
            aim += int(s[1])
        elif s[0] == 'up':
            aim -= int(s[1])
    print(horz * vert)


if __name__ == '__main__':
    main()
