
import string


def main():
    content = []
    with open('input', 'r') as f:
        content = f.read().splitlines()
    priority_sum = 0

    prio_list = [chr(i) for i in range(ord('a'), ord('z') + 1)] + \
        [chr(i) for i in range(ord('A'), ord('Z') + 1)]

    for line in content:
        hl = len(line) >> 1
        result = next(iter(set(line[:hl]).intersection(line[hl:])))
        priority_sum += (prio_list.index(result) + 1)

    print('Sum of priority elements {}'.format(priority_sum))
    badge_sum = 0
    sublist = [content[x:x+3] for x in range(0, len(content), 3)]
    for s in sublist:
        r = next(iter(set(s[0]) & set(s[1]) & set(s[2])))
        badge_sum += (prio_list.index(r) + 1)
    print('Sum of badge elements {}'.format(badge_sum))

if __name__ == '__main__':
    main()
