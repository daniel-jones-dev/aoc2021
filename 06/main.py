

def main():
    fish = [int(s) for s in open("input.txt").readline().split(",")]

    fish_counts = dict(
        [(age, sum(1 for f in fish if f == age)) for age in range(9)]
    )

    last_day = 256
    for day_i in range(1000):
        if day_i == last_day:
            print(sum(fish_counts.values()))
            break

        new_fish = fish_counts[0]

        for age in range(8):
            fish_counts[age] = fish_counts[age+1]
        fish_counts[6] += new_fish
        fish_counts[8] = new_fish


if __name__ == "__main__":
    main()
