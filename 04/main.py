def print_board_score(board, drawn_num):
    sum_unmarked = sum([(num if not picked else 0) for picked, num in board])
    print(sum_unmarked * drawn_num)


def is_board_complete(board, new_index):
    row = [5 * x + new_index % 5 for x in range(5)]
    col = [5 * int(new_index / 5) + x for x in range(5)]
    return all([board[check][0] for check in row]) or all(
        [board[check][0] for check in col]
    )


def main():
    with open("input.txt") as f:
        drawn_nums = [int(s) for s in f.readline().split(",")]
        boards = []
        while True:
            board_str = ""
            for i in range(6):
                board_str += " " + f.readline().strip()
            board_str = board_str.replace("  ", " ").strip()
            if len(board_str) == 0:
                break
            boards.append([(False, int(s)) for s in board_str.split(" ")])

    part_1_done = False
    for drawn_num in drawn_nums:
        board_index = 0
        while board_index < len(boards):
            board_str = boards[board_index]
            for i in range(25):
                _, num = board_str[i]
                if num == drawn_num:
                    board_str[i] = True, num
                    if is_board_complete(board_str, i):
                        if not part_1_done:
                            part_1_done = True
                            print_board_score(board_str, drawn_num)
                        elif len(boards) == 1:
                            print_board_score(board_str, drawn_num)
                            exit(0)
                        boards.pop(board_index)
                    else:
                        board_index += 1
                    break
            else:
                board_index += 1

    exit(1)


if __name__ == "__main__":
    main()
