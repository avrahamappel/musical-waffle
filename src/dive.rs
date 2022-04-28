trait Plotter {
    fn plot<I>(mut self, instructions: I) -> u32
    where
        I: Iterator<Item = String>,
        Self: Sized,
    {
        for instruction in instructions {
            self = self.apply(Instruction::parse(instruction))
        }

        self.multiply()
    }

    fn apply(self, instruction: Instruction) -> Self;

    fn multiply(self) -> u32;
}

struct Submarine {
    depth: u32,
    across: u32,
}

impl Submarine {
    fn new() -> Self {
        Submarine {
            depth: 0,
            across: 0,
        }
    }
}

impl Plotter for Submarine {
    fn apply(mut self, instruction: Instruction) -> Self {
        let Instruction(cmd, num) = instruction;

        match cmd {
            Command::Up => self.depth -= num,
            Command::Down => self.depth += num,
            Command::Forward => self.across += num,
        }

        self
    }

    fn multiply(self) -> u32 {
        self.depth * self.across
    }
}

struct AimedSubmarine {
    depth: u32,
    across: u32,
    aim: u32,
}

impl AimedSubmarine {
    fn new() -> Self {
        AimedSubmarine {
            depth: 0,
            across: 0,
            aim: 0,
        }
    }
}

impl Plotter for AimedSubmarine {
    fn apply(mut self, instruction: Instruction) -> Self {
        let Instruction(cmd, num) = instruction;

        match cmd {
            Command::Up => self.aim -= num,
            Command::Down => self.aim += num,
            Command::Forward => {
                self.across += num;
                self.depth += self.aim * num;
            }
        }

        self
    }

    fn multiply(self) -> u32 {
        self.depth * self.across
    }
}

struct Instruction(Command, u32);

impl Instruction {
    fn parse(instr: String) -> Self {
        let cmd = Self::parse_cmd(&instr).expect("Invalid command");
        let number = Self::parse_num(&instr).expect("Invalid command");

        Self(cmd, number)
    }

    fn parse_cmd(instr: &str) -> Option<Command> {
        match &instr.matches(char::is_alphabetic).collect::<String>()[..] {
            "up" => Some(Command::Up),
            "down" => Some(Command::Down),
            "forward" => Some(Command::Forward),
            _ => None,
        }
    }

    fn parse_num(instr: &str) -> Option<u32> {
        instr
            .matches(char::is_numeric)
            .map(|s| s.parse::<u32>().ok())
            .collect::<Vec<_>>()[0]
    }
}

enum Command {
    Up,
    Down,
    Forward,
}

pub fn plot_course<I>(instructions: I) -> u32
where
    I: Iterator<Item = String>,
{
    Submarine::new().plot(instructions)
}

pub fn plot_aimed_course<I>(instructions: I) -> u32
where
    I: Iterator<Item = String>,
{
    AimedSubmarine::new().plot(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

    #[test]
    fn it_plots_the_course_from_instructions() {
        assert_eq!(150, plot_course(DATA.lines().map(Into::into)))
    }

    #[test]
    fn it_plots_the_aimed_course_from_instructions() {
        assert_eq!(900, plot_aimed_course(DATA.lines().map(Into::into)))
    }
}
