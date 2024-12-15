import sys

def read_file():
    func = lambda x : x.strip()
    with open('input.txt') as f:
        content = f.readlines()
        return list(map(func, content))

if __name__ == "__main__":
    _slice = read_file()
    slice_height = len(_slice)
    slice_width = len(_slice[0])
    routes = ((1,1), (3,1), (5,1), (7,1), (1,2))
    trees = 1
    for route in routes:
        step = route[0]
        index = step
        tree = 0
        for line in _slice[slice(route[1], len(_slice), route[1])]:
            if line[index % slice_width] == '#':
                tree += 1
            index += step
        trees *= tree
    print(trees)