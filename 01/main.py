def main():
    last_num = None
    inc_count = 0
    with open("input.txt") as f:
        for line in f.readlines():
            num = int(line)
            if last_num is not None and num > last_num:
                inc_count += 1
            last_num = num

    print(inc_count)
    exit(0)


if __name__ == "__main__":
    main()
