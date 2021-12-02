def main():
    pos, depth, aim = 0, 0, 0

    with open("input.txt") as f:
        for line in f.readlines():
            dir, num = line.split(" ")
            num = int(num)
            if dir == "forward":
                pos += num
                depth += aim * num
            elif dir == "down":
                aim += num
            else:
                aim -= num
    print(depth * pos)


if __name__ == "__main__":
    main()
