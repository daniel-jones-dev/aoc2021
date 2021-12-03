def drop_lines(lines: list[str], pos: int, keep_value: str):
    i = 0
    while i < len(lines) and len(lines) > 1:
        if lines[i][pos] != keep_value:
            lines.pop(i)
        else:
            i += 1


def count_values(lines: list[str]):
    sz = len(lines[0]) - 1  # minus 1 for newline
    zero_counts = [0] * sz

    for line in lines:
        for pos in range(sz):
            if line[pos] == "0":
                zero_counts[pos] += 1

    return zero_counts, [len(lines) - z for z in zero_counts], sz


def main():
    with open("input.txt") as f:
        lines = f.readlines()

    num_zeros, num_ones, sz = count_values(lines)

    gamma_str = "".join(
        ["0" if z > o else "1" for z, o in zip(num_zeros, num_ones)]
    )
    gamma = int(gamma_str, 2)
    epsilon = 2 ** sz - 1 - gamma
    print(gamma * epsilon)

    # part 2
    oxy_lines = list(lines)
    co2_lines = list(lines)

    for invert, lines in [(False, oxy_lines), (True, co2_lines)]:
        for pos in range(sz):
            if len(lines) == 1:
                break
            num_zeros, num_ones, _ = count_values(lines)
            if (num_zeros[pos] > num_ones[pos]) != invert:  # XOR using ==
                drop_lines(lines, pos, "0")
            else:
                drop_lines(lines, pos, "1")

    oxy = int(oxy_lines[0], 2)
    co2 = int(co2_lines[0], 2)
    print(oxy * co2)


if __name__ == "__main__":
    main()
