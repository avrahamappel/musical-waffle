use std::collections::HashSet;

const BOARD_LENGTH: usize = 5;

struct Board {
    grid: [[u32; BOARD_LENGTH]; BOARD_LENGTH],
    circled: HashSet<(usize, usize)>,
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..BOARD_LENGTH {
            write!(f, "\n")?;
            for y in 0..BOARD_LENGTH {
                if self.circled.contains(&(x, y)) {
                    write!(f, "({:>2}) ", self.grid[x][y])?;
                } else {
                    write!(f, " {:>2}  ", self.grid[x][y])?;
                }
            }
            write!(f, "\n")?;
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
        }
    }

    fn play_turn(&mut self, num: u32) -> Option<u32> {
        if let Some((x, y)) = self
            .grid
            .iter()
            .enumerate()
            .find_map(|(x, row)| row.iter().position(|n| *n == num).map(|y| (x, y)))
        {
            self.circled.insert((x, y));

            if self.check_for_win(x, y) {
                return Some(self.calculate_score(num));
            }
        }
        None
    }

    fn check_for_win(&self, x: usize, y: usize) -> bool {
        (0..BOARD_LENGTH).all(|i| self.circled.contains(&(x, i)))
            || (0..BOARD_LENGTH).all(|i| self.circled.contains(&(i, y)))
    }

    fn calculate_score(&self, winning: u32) -> u32 {
        let iter = || 0..BOARD_LENGTH;

        iter()
            .flat_map(|x| {
                iter().flat_map(move |y| (!self.circled.contains(&(x, y))).then(|| self.grid[x][y]))
            })
            .sum::<u32>()
            * winning
    }
}

struct Bingo {
    numbers: Vec<u32>,
    boards: Vec<Board>,
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

        let boards = data.map(|b| Board::new(b)).collect();

        Bingo { numbers, boards }
    }

    fn play(mut self) -> Vec<u32> {
        let mut scores = Vec::with_capacity(self.boards.len());

        for n in self.numbers {
            for b in &mut self.boards {
                if let Some(scr) = b.play_turn(n) {
                    scores.push(scr);
                }
            }
        }

        scores
    }
}

pub fn winning_board_score(data: &str) -> Option<u32> {
    Bingo::new(data).play().first().map(|u| u.to_owned())
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
    fn it_calculates_winning_board_score() {
        assert_eq!(Some(4512), winning_board_score(DATA))
    }
}
