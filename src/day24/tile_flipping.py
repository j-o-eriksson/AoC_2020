
#

from data import input

dict1 = {'e': (1, 0), 'w': (-1, 0)}
dict2 = {'se': (0, 1), 'sw': (-1, 1), 'nw': (0, -1), 'ne':(1, -1)}

neighbors = [v for _, v in dict1.items()] + [v for _, v in dict2.items()] 


def parse_chars(cs):
    c = cs[0]

    if c in dict1:
        return dict1[c], cs[1:]

    # error check?
    s = c + cs[1]

    if s in dict2:
        return dict2[s], cs[2:]
    else:
        raise ValueError


def parse_line(line):    
    dirs = []
    cs = line

    while True:
        d0, cs = parse_chars(cs)
        dirs.append(d0)

        if not cs:
            break   

    return dirs


def get_relative_index(path):
    x = 0
    y = 0

    for dx, dy in path:
        x += dx
        y += dy

    return x, y


def get_neighbors(grid, x, y):
    return [grid[x + nx][y + ny] for nx, ny in neighbors]


def simulate_floor(input_grid, size, n_days):
    grid = [l.copy() for l in input_grid]

    for _ in range(n_days):
        old_grid = [l.copy() for l in grid]

        for i in range(1, size - 1):
            for j in range(1, size - 1):
                neighbors = get_neighbors(old_grid, i, j)
                val = old_grid[i][j]

                if val == 0: # tile iswhite
                    if sum(neighbors) == 2:
                        grid[i][j] = 1
                else: # tile is black
                    if sum(neighbors) == 0 or sum(neighbors) > 2:
                        grid[i][j] = 0        

    return grid


# load data

lines = input.split('\n')
paths = [parse_line(line) for line in lines]

tiles = [get_relative_index(path) for path in paths]

# initialize grid

size = 150  
offset = 75

hexgrid = [size * [0] for _ in range(size)]

for x, y in tiles:
    val = hexgrid[x + offset][y + offset]
    new_val = 1 if val == 0 else 0
    hexgrid[x + offset][y + offset] = new_val

# part 1

print(sum(sum(c) for c in hexgrid))

# part 2

final_grid = simulate_floor(hexgrid, size, 100)
print(sum(sum(c) for c in final_grid))
