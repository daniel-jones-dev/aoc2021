def cost_at(align, positions):
    return sum([abs(p - align) for p in positions])


def main():
    positions = [int(s) for s in open("input.txt").readline().split(",")]

    guess = int(sum(positions) / len(positions))
    best_cost = cost_at(guess, positions)

    curr = guess + 1
    cost = cost_at(curr, positions)
    while cost < best_cost:
        curr += 1
        best_cost, cost = cost, cost_at(curr, positions)

    curr = guess - 1
    cost = cost_at(curr, positions)
    while cost < best_cost:
        curr -= 1
        best_cost, cost = cost, cost_at(curr, positions)

    print(best_cost)


if __name__ == "__main__":
    main()
