def found_winning_board(board, drawn_num, found_index):
    sum_unmarked = sum([(num if not picked else 0) for picked, num in board])
    print(sum_unmarked * drawn_num)
    exit(0)


def main():
    with open("input.txt") as f:
        drawn_nums = [int(s) for s in f.readline().split(",")]
        boards = []
        while True:
            board = ""
            for i in range(6):
                board += " " + f.readline().strip()
            board = board.replace("  ", " ").strip()
            if len(board) == 0:
                break
            boards.append([(False, int(s)) for s in board.split(" ")])

    for drawn_num in drawn_nums:
        for board in boards:
            for i in range(25):
                _, num = board[i]
                if num == drawn_num:
                    board[i] = True, num
                    if all(
                        [
                            board[c][0]
                            for c in [5 * x + i % 5 for x in range(5)]
                        ]
                    ) or all(
                        [
                            board[c][0]
                            for c in [int(i / 5) + x for x in range(5)]
                        ]
                    ):
                        found_winning_board(board, drawn_num, i)
    exit(1)


if __name__ == "__main__":
    main()
