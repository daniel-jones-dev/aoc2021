use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut total_syntax_error_score = 0;
    let mut completion_scores = Vec::new();
    'each_line: for (_, line) in contents.lines().enumerate() {
        let mut stack = Vec::new();
        for (_, char) in line.chars().enumerate() {
            if "([<{".contains(char) {
                stack.push(char);
            } else {
                let last = *stack.last().unwrap_or(&' ');
                let (corrupt, syntax_error_score) = match char {
                    ')' => (last != '(', 3),
                    ']' => (last != '[', 57),
                    '}' => (last != '{', 1197),
                    '>' => (last != '<', 25137),
                    _ => (true, 0)
                };
                if corrupt {
                    total_syntax_error_score += syntax_error_score;
                    continue 'each_line;
                } else {
                    stack.pop();
                }
            }
        }

        let mut completion_score = 0u64;
        while !stack.is_empty() {
            completion_score *= 5;
            completion_score += match stack.last().unwrap() {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!()
            };
            stack.pop();
        }
        completion_scores.push(completion_score);
    }
    completion_scores.sort();

    println!("{}", total_syntax_error_score);

    let middle_score = completion_scores[(completion_scores.len() - 1) / 2];
    println!("{}", middle_score);
}
