def main():
    pos, depth = 0, 0

    with open("input.txt") as f:
        for line in f.readlines():
            dir, num = line.split(" ")
            num = int(num)
            if dir == "forward":
                pos += num
            elif dir == "down":
                depth += num
            else:
                depth -= num
    print(depth * pos)


if __name__ == "__main__":
    main()
