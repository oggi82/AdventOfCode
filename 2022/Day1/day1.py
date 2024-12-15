
def part1():
    content = []
    try:
        with open('input', 'r') as file:
            content = file.readlines()
    except Exception as e:
        print("Error occured {}", e)
    sum_calories = 0
    calories = []
    for c in content:
        if not c.strip():
            calories.append(sum_calories)
            sum_calories = 0
            continue
        sum_calories += int(c.strip())#
        
    print("elf with max calories {}".format(max(calories)))
    sorted_calories = sorted(calories)
    print("Sum of calories by the top three elfs {}".format(sum(sorted_calories[-3:])))

if __name__ == '__main__':
    part1()
