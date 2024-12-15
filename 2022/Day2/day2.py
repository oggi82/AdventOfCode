
from collections import namedtuple

Player1 = namedtuple('Player1', ['A', 'B', 'C'])
Player2 = namedtuple('Player2', ['X', 'Y', 'Z'])

player1 = Player1('Rock', 'Paper', 'Scissor')
player2 = Player2('Rock', 'Paper', 'Scissor')

ScoreShape = namedtuple('ScoreShape', ['Rock', 'Paper', 'Scissor'])
scoreShape = ScoreShape(1, 2, 3)

ScoreOutcome = namedtuple('ScoreOutcome', ['win', 'draw', 'lost'])
scoreOutcome = ScoreOutcome(6, 3, 0)

def calcOneGamePart2(input):
    p1 = getattr(player1, input[0])
    p2 = getattr(player2, input[1])
    
    match input[1]:
        case 'X':   # need to loss
            if p1 == 'Rock':
                return [0, scoreShape.Scissor]
            elif p1 == 'Scissor':
                return [0, scoreShape.Paper]
            elif p1 == 'Paper':
                return [0, scoreShape.Rock]
        case 'Y':   # ends in a draw
            return [3, getattr(scoreShape, p1)] # draw
        case 'Z':   # need to win
            if p1 == 'Rock':
                return [6, scoreShape.Paper]
            elif p1 == 'Scissor':
                return [6, scoreShape.Rock]
            elif p1 == 'Paper':
                return [6, scoreShape.Scissor]
        case _:
            pass
    return [0, 0]

def calcOneGamePart1(input):
    p1 = getattr(player1, input[0])
    p2 = getattr(player2, input[1])

    if p1 == p2:
        return [3, getattr(scoreShape, p2)] # draw
    elif ((p2 == 'Rock' and p1 == 'Scissor') or
    (p2 == 'Scissor' and p1 == 'Paper') or
    (p2 == 'Paper' and p1 == 'Rock')):
        return [6, getattr(scoreShape, p2)] # win
    else:
        return [0, getattr(scoreShape, p2)] # lost

def main():
    content = []
    with open('input', 'r') as f:
        content = f.read().splitlines()
    result_1 = 0
    result_2 = 0
    for c in content:
        r = calcOneGamePart1(c.split())
        result_1 += sum(r)
        r = calcOneGamePart2(c.split())
        result_2 += sum(r)
    print('Part1 {}'.format(result_1))
    print('Part2 {}'.format(result_2))

if __name__ == '__main__':
    main()
