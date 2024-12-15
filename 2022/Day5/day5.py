
def readfile():
    with open('input.txt') as fd:
        #return fd.read().splitlines()
        return [line.rstrip() for line in fd]

def task1():
    fd = open('input.txt', 'r')
    levels, instruction = [sections.split('\n') for sections in fd.read().split('\n\n')]
    print(levels)
if __name__ == '__main__':
    task1()