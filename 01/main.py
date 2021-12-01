def triplewise(it):
    try:
        a, b = next(it), next(it)
    except StopIteration:
        return
    for c in it:
        yield a, b, c
        a, b = b, c


def inc_count(values):
    last = None
    count = 0
    for v in values:
        if last is not None and v > last:
            count += 1
        last = v
    return count


def main():
    nums = [int(line) for line in open("input.txt").readlines()]
    sums = [sum(triple) for triple in triplewise(iter(nums))]
    print(inc_count(nums))  # part 1
    print(inc_count(sums))  # part 2


if __name__ == "__main__":
    main()
