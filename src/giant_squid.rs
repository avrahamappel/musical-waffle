use std::collections::HashSet;

const BOARD_LENGTH: usize = 5;

struct Board {
    grid: [[u32; BOARD_LENGTH]; BOARD_LENGTH],
    circled: HashSet<(usize, usize)>,
    won: bool,
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..BOARD_LENGTH {
            writeln!(f)?;
            for y in 0..BOARD_LENGTH {
                if self.circled.contains(&(x, y)) {
                    write!(f, "({:>2}) ", self.grid[x][y])?;
                } else {
                    write!(f, " {:>2}  ", self.grid[x][y])?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    fn new(data: &str) -> Self {
        let mut iter = data.lines().map(|l| {
            let mut it = l
                .split_whitespace()
                .map(|d| d.parse::<u32>().expect("Board data contained"));
            [(); BOARD_LENGTH].map(|_| it.next().expect("Board not wide enough"))
        });
        let grid = [(); BOARD_LENGTH].map(|_| iter.next().expect("Board not tall enough"));

        Board {
            grid,
            circled: HashSet::new(),
            won: false,
        }
    }

    fn play_turn(mut self, num: u32) -> (Self, Option<u32>) {
        if self.won {
            return (self, None);
        }

        if let Some((x, y)) = self
            .grid
            .iter()
            .enumerate()
            .find_map(|(x, row)| row.iter().position(|n| *n == num).map(|y| (x, y)))
        {
            self.circled.insert((x, y));

            if self.check_for_win(x, y) {
                self.won = true;
                let score = self.calculate_score(num);

                return (self, Some(score));
            }
        }

        (self, None)
    }

    fn check_for_win(&self, x: usize, y: usize) -> bool {
        (0..BOARD_LENGTH).all(|i| self.circled.contains(&(x, i)))
            || (0..BOARD_LENGTH).all(|i| self.circled.contains(&(i, y)))
    }

    fn calculate_score(&self, winning: u32) -> u32 {
        let iter = || 0..BOARD_LENGTH;

        iter()
            .flat_map(|x| {
                iter()
                    .filter_map(move |y| (!self.circled.contains(&(x, y))).then(|| self.grid[x][y]))
            })
            .sum::<u32>()
            * winning
    }
}

struct Boards {
    boards: Vec<Board>,
}

impl Boards {
    fn new(data: &[&str]) -> Self {
        let boards = data.iter().copied().map(Board::new).collect();
        Self { boards }
    }

    fn play_number(mut self, num: u32) -> (Self, Vec<u32>) {
        let (boards, scores): (Vec<Board>, Vec<Option<u32>>) =
            self.boards.into_iter().map(|b| b.play_turn(num)).unzip();

        self.boards = boards;

        (self, scores.into_iter().flatten().collect())
    }
}

struct Bingo {
    numbers: Vec<u32>,
    boards: Boards,
}

impl Bingo {
    fn new(data: &str) -> Self {
        let mut data = data.split("\n\n");

        let numbers = data
            .next()
            .expect("Empty numbers line")
            .trim()
            .split(',')
            .map(|s| s.parse().expect("Numbers line contained unexpected"))
            .collect();

        let boards = Boards::new(&data.collect::<Vec<_>>());

        Bingo { numbers, boards }
    }

    fn play(self) -> Vec<u32> {
        let (_, scores) =
            self.numbers
                .into_iter()
                .fold((self.boards, Vec::new()), |(boards, mut scores), n| {
                    let (boards, mut scrs) = boards.play_number(n);
                    scores.append(&mut scrs);
                    (boards, scores)
                });

        scores
    }
}

pub fn first_winning_board_score(data: &str) -> Option<u32> {
    match Bingo::new(data).play()[..] {
        [head, ..] => Some(head),
        _ => None,
    }
}

pub fn last_winning_board_score(data: &str) -> Option<u32> {
    match Bingo::new(data).play()[..] {
        [.., last] => Some(last),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
 2  0 12  3  7"#;

    #[test]
    fn it_calculates_first_winning_board_score() {
        assert_eq!(Some(4512), first_winning_board_score(DATA));
    }

    #[test]
    fn it_calculates_last_winning_board_score() {
        assert_eq!(Some(1924), last_winning_board_score(DATA));
    }
}
