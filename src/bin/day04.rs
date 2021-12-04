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

    fn play(&mut self) -> (Option<Board>, u32) {
        for number in &self.draws {
            for board in &mut self.boards {
                board.mark(*number);
                if board.is_winner() {
                    // FIXME figure out how to return a reference that lives long enough instead of cloning.
                    return (Some(board.clone()), *number);
                }
            }
        }
        (None, 0)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct MarkedNumber(u32, bool);

impl MarkedNumber {
    fn new(n: u32) -> Self {
        MarkedNumber(n, false)
    }
    fn mark(&mut self) {
        self.1 = true;
    }
    fn unmark(&mut self) {
        self.1 = false;
    }
}

impl From<u32> for MarkedNumber {
    fn from(n: u32) -> Self {
        MarkedNumber::new(n)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Board {
    rows: Vec<Vec<MarkedNumber>>,
}

impl Board {
    fn new(rows: Vec<Vec<u32>>) -> Self {
        Board {
            rows: rows
                .into_iter()
                .map(|row| row.into_iter().map(|n| n.into()).collect())
                .collect(),
        }
    }

    fn mark(&mut self, drawn_number: u32) {
        for row in &mut self.rows {
            if let Some(marked_number) =
                row.iter_mut()
                    .find_map(|n| if n.0 == drawn_number { Some(n) } else { None })
            {
                (*marked_number).mark();
                break;
            }
        }
    }

    fn is_winner(&self) -> bool {
        for (i, row) in self.rows.iter().enumerate() {
            // Check the row
            if row.iter().all(|n| n.1 == true) {
                return true;
            }

            // Check the column
            if self.rows.iter().map(|r| &r[i]).all(|n| n.1 == true) {
                return true;
            }
        }

        false
    }

    fn score(&self) -> u32 {
        self.rows
            .iter()
            .flatten()
            .filter_map(|MarkedNumber(n, marked)| if *marked { None } else { Some(n) })
            .sum()
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

    #[test]
    fn it_marks_numbers_in_board() {
        let mut board = Board::new(vec![vec![1, 2, 3]]);

        board.mark(1);
        assert_eq!(
            board.rows,
            vec![vec![
                MarkedNumber(1, true),
                MarkedNumber(2, false),
                MarkedNumber(3, false)
            ]]
        )
    }

    #[test]
    fn it_marks_board_as_winner_when_row_is_full() {
        let mut board = Board::new(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert!(!board.is_winner());
        board.mark(1);
        assert!(!board.is_winner());
        board.mark(2);
        board.mark(3);
        assert!(board.is_winner());
    }

    #[test]
    fn it_marks_board_as_winner_when_column_is_full() {
        let mut board = Board::new(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert!(!board.is_winner());
        board.mark(1);
        board.mark(4);
        assert!(board.is_winner());
    }

    #[test]
    fn it_plays_until_a_board_wins() {
        let mut bingo_game = BingoGame::from(TEST_INPUT);

        let (winning_board, last_number) = bingo_game.play();
        assert_eq!(last_number, 24);
        assert_eq!(winning_board.unwrap().score(), 188);
    }
}
