def part1_cost_at(align, positions):
    return sum([abs(p - align) for p in positions])


def part2_cost_at(align, positions):
    def cost(p):
        d = abs(p-align)
        return int(d*(d+1)/2)
    return sum([cost(p) for p in positions])


def main():
    positions = [int(s) for s in open("input.txt").readline().split(",")]

    # Starting guess at arithmetic mean
    guess = int(sum(positions) / len(positions))
    best_cost = part1_cost_at(guess, positions)

    # Search up and down until cost stops improving
    curr = guess + 1
    cost = part1_cost_at(curr, positions)
    while cost < best_cost:
        curr += 1
        best_cost, cost = cost, part1_cost_at(curr, positions)

    curr = guess - 1
    cost = part1_cost_at(curr, positions)
    while cost < best_cost:
        curr -= 1
        best_cost, cost = cost, part1_cost_at(curr, positions)

    print(best_cost)

    # part 2 search
    best_cost = part2_cost_at(guess, positions)
    curr = guess + 1
    cost = part2_cost_at(curr, positions)
    while cost < best_cost:
        curr += 1
        best_cost, cost = cost, part2_cost_at(curr, positions)

    curr = guess - 1
    cost = part2_cost_at(curr, positions)
    while cost < best_cost:
        curr -= 1
        best_cost, cost = cost, part2_cost_at(curr, positions)

    print(best_cost)


if __name__ == "__main__":
    main()
