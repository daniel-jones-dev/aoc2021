def in_but_not_in(in_this, not_in_this):
    return "".join([x for x in in_this if x not in not_in_this])


def in_and_in(in_this, in_this_too):
    return "".join([x for x in in_this if x in in_this_too])


def main():
    special_digit_lengths = [2, 3, 4, 7]
    special_output_count = 0

    solved_output_sum = 0

    with open("input.txt") as f:
        for line in f.readlines():
            unique_signals, outputs = [
                ["".join(sorted(s)) for s in part.strip().split(" ")]
                for part in line.split("|")
            ]
            for output in outputs:
                if len(output) in special_digit_lengths:
                    special_output_count += 1
            unique_signals.sort(key=len)

            #  AA
            # B  C
            #  DD
            # E  F
            #  GG
            cf = unique_signals[0]  # The 1 digit
            acf = unique_signals[1]  # The 7 digit
            bcdf = unique_signals[2]  # The 4 digit
            # The 2, 3, 5 digits have five bars each
            fives = [unique_signals[3], unique_signals[4], unique_signals[5]]
            # The 6, 9, 0 digits have six bars each
            sixes = [unique_signals[6], unique_signals[7], unique_signals[8]]
            # We can ignore the 8 digit, it gives no clues

            # A appears in ACF but not CF
            a = in_but_not_in(acf, cf)

            # The three bars that appear in 2,3,5 are ADG
            adg = "".join(
                [x for x in "abcdefg" if all([x in f for f in fives])]
            )
            # DG are in ADG but not A
            dg = in_but_not_in(adg, a)
            # dg = "".join([x for x in adg if x != a])

            # The two bars that appear only once in 2,3,5 are BE
            be = "".join(c for c in "abcdefg" if "".join(fives).count(c) == 1)

            # The bars that appear only twice in 6,9,0 are CDE
            cde = "".join(c for c in "abcdefg" if "".join(sixes).count(c) == 2)

            d = in_and_in(cde, dg)
            c = in_and_in(cf, cde)
            b = in_and_in(be, bcdf)
            e = in_but_not_in(be, b)
            f = in_but_not_in(cf, c)
            g = in_but_not_in(dg, d)

            #  AA
            # B  C
            #  DD
            # E  F
            #  GG
            key = {
                "".join(sorted([c, f])): "1",
                "".join(sorted([a, c, d, e, g])): "2",
                "".join(sorted([a, c, d, f, g])): "3",
                "".join(sorted([b, c, d, f])): "4",
                "".join(sorted([a, b, d, f, g])): "5",
                "".join(sorted([a, b, d, e, f, g])): "6",
                "".join(sorted([a, c, f])): "7",
                "".join(sorted([a, b, c, d, e, f, g])): "8",
                "".join(sorted([a, b, c, d, f, g])): "9",
                "".join(sorted([a, b, c, e, f, g])): "0",
            }

            ans = int("".join([key[output] for output in outputs]))
            solved_output_sum += ans

    print(special_output_count)
    print(solved_output_sum)

if __name__ == "__main__":
    main()
