def main():
    with open("input.txt") as f:
        nums = [int(line) for line in f.readlines()]

    for num in nums:
        for num2 in nums:
            if num + num2 == 2020:
                print(num * num2)
                exit(0)
    exit(1)


if __name__ == "__main__":
    main()
