import sys
import re

str_to_num = {
    'one': 'o1e',
    'two': 't2o',
    'three': 't3ree',
    'four': 'f4ur',
    'five': 'f5ve',
    'six': 's6x',
    'seven': 's7ven',
    'eight': 'e8ght',
    'nine': 'n9ne',
}


def replaceWords(l):
    for k, v in str_to_num.items():
        l = l.replace(k, v)
    return l


def solution1():
    sum = 0
    for line in open('day1a.txt'):
        m = re.findall(r'\d', line)
        sum += int(m[0] + m[-1])
    print(sum)


def solution2():
    sum = 0
    for line in open('day1a.txt'):
        line = replaceWords(line)
        m = re.findall(r'\d', line)
        sum += int(m[0] + m[-1])
    print(sum)


if __name__ == '__main__':
    solution1()
    solution2()
