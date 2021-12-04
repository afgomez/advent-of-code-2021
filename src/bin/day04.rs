use aoc::input::read_input;

struct BingoGame {
    draws: Vec<u32>,
    boards: Vec<Board>,
}

impl BingoGame {
    fn from<T: AsRef<str>>(input: T) -> BingoGame {
        let input = input.as_ref();

        if input.len() == 0 {
            return BingoGame {
                draws: vec![],
                boards: vec![],
            };
        }

        let mut lines = input.lines();

        // First line contains the number draws
        let draws: Vec<u32> = lines
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        // Drop empty whitespace
        lines.next();

        let mut boards: Vec<Board> = vec![];
        let mut rows: Vec<Vec<u32>> = vec![];

        while let Some(line) = lines.next() {
            let line = line.trim();

            // Create a board with the collected rows when we find an empty line
            if line.len() == 0 {
                let board = Board::new(rows.to_vec());
                boards.push(board);
                rows.clear();
            } else {
                let row: Vec<u32> = line
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                rows.push(row);
            }
        }

        // Make board with the last rows
        if rows.len() > 0 {
            let board = Board::new(rows.to_vec());
            boards.push(board);
        }

        BingoGame { draws, boards }
    }
}

#[derive(Debug, PartialEq)]
struct Board {
    rows: Vec<Vec<u32>>,
}

impl Board {
    fn new(rows: Vec<Vec<u32>>) -> Self {
        Board { rows }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn it_parses_game_input() {
        let bingo_game = BingoGame::from(TEST_INPUT);

        assert_eq!(
            bingo_game.draws,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
        assert_eq!(
            bingo_game.boards,
            vec![
                Board::new(vec![
                    vec![22, 13, 17, 11, 0],
                    vec![8, 2, 23, 4, 24],
                    vec![21, 9, 14, 16, 7],
                    vec![6, 10, 3, 18, 5],
                    vec![1, 12, 20, 15, 19],
                ]),
                Board::new(vec![
                    vec![3, 15, 0, 2, 22],
                    vec![9, 18, 13, 17, 5],
                    vec![19, 8, 7, 25, 23],
                    vec![20, 11, 10, 24, 4],
                    vec![14, 21, 16, 12, 6],
                ]),
                Board::new(vec![
                    vec![14, 21, 17, 24, 4],
                    vec![10, 16, 15, 9, 19],
                    vec![18, 8, 23, 26, 20],
                    vec![22, 11, 13, 6, 5],
                    vec![2, 0, 12, 3, 7],
                ])
            ]
        );
    }
}
