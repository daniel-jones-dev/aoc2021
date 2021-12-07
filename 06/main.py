fish = []


def print_fish(day_i):
    global fish
    label = "Initial state: " if day_i == 0 else f"After {day_i:02} days: "
    print(label + ",".join(map(str, fish)))


def main():
    global fish
    fish = [int(s) for s in open("input.txt").readline().split(",")]

    last_day = 80

    for day_i in range(1000):
        print_fish(day_i)
        if day_i == last_day:
            print(len(fish))
            break
        num_fish = len(fish)
        for i in range(num_fish):
            age = fish[i]
            if age == 0:
                new_age = 6
                fish.append(8)
            else:
                new_age = age - 1
            fish[i] = new_age


if __name__ == "__main__":
    main()
