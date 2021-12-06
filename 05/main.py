def line_cells(line):
    x1, y1, x2, y2 = line
    count = max(abs(x2 - x1), abs(y2 - y1)) + 1

    def coords(v1, v2):
        if v2 == v1:
            return [v1 for _ in range(count)]
        else:
            s = 1 if v2 > v1 else -1
            return range(v1, v2 + s, s)

    return zip(coords(x1, x2), coords(y1, y2))


def print_map_value_count(map_values):
    print(sum(sum([1 if v >= 2 else 0 for v in m]) for m in map_values))


def add_lines_to_map_and_print(map_values, lines):
    for line in lines:
        for x, y in line_cells(line):
            map_values[y][x] += 1
    print_map_value_count(map_values)


def main():
    lines = []
    with open("input.txt") as f:
        for line in f.readlines():
            parts = line.replace(" -> ", ",").split(",")
            x1, y1, x2, y2 = map(int, parts)
            lines.append((x1, y1, x2, y2))

    non_diag_lines = [
        (x1, y1, x2, y2) for x1, y1, x2, y2 in lines if x1 == x2 or y1 == y2
    ]
    diag_lines = [
        (x1, y1, x2, y2) for x1, y1, x2, y2 in lines if x1 != x2 and y1 != y2
    ]

    max_x = max(max(x1, x2) for x1, y1, x2, y2 in lines) + 1
    max_y = max(max(y1, y2) for x1, y1, x2, y2 in lines) + 1
    map_values = [[0 for _ in range(max_x)] for _ in range(max_y)]

    add_lines_to_map_and_print(map_values, non_diag_lines)
    add_lines_to_map_and_print(map_values, diag_lines)


if __name__ == "__main__":
    main()
