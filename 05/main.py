def line_cells(x1, y1, x2, y2):
    if x1 == x2:
        if y2 > y1:
            return [(x1, y) for y in range(y1, y2 + 1)]
        else:
            return [(x1, y) for y in range(y2, y1 + 1)]
    else:
        if x2 > x1:
            return [(x, y1) for x in range(x1, x2 + 1)]
        else:
            return [(x, y1) for x in range(x2, x1 + 1)]


def main():
    lines = []
    with open("input.txt") as f:
        for line in f.readlines():
            parts = line.replace(" -> ", ",").split(",")
            x1, y1, x2, y2 = map(int, parts)
            lines.append((x1, y1, x2, y2))

    # remove diagonal lines
    lines = [
        (x1, y1, x2, y2) for x1, y1, x2, y2 in lines if x1 == x2 or y1 == y2
    ]

    max_x = max(max(x1, x2) for x1, y1, x2, y2 in lines) + 1
    max_y = max(max(y1, y2) for x1, y1, x2, y2 in lines) + 1

    map_values = [[0 for _ in range(max_x)] for _ in range(max_y)]

    for line in lines:
        x1, y1, x2, y2 = line
        for x, y in line_cells(x1, y1, x2, y2):
            map_values[y][x] += 1

    print(sum(sum([1 if v >= 2 else 0 for v in m]) for m in map_values))

    exit(0)


if __name__ == "__main__":
    main()
