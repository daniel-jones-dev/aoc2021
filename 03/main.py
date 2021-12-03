def main():

    sz = None
    zero_counts, one_counts = None, None

    with open("input.txt") as f:
        for line in f.readlines():
            if sz is None:
                sz = len(line) - 1
                zero_counts = [0] * sz
                one_counts = [0] * sz
            for i in range(sz):
                if line[i] == "0":
                    zero_counts[i] += 1
                else:
                    one_counts[i] += 1

    gamma_str = "".join(
        ["0" if z > o else "1" for z, o in zip(zero_counts, one_counts)]
    )
    gamma = int(gamma_str, 2)
    epsilon = 2 ** sz - 1 - gamma
    print(gamma * epsilon)


if __name__ == "__main__":
    main()
