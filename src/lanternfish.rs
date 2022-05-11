use futures::future::join_all;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list1;

type Error<T> = nom::Err<nom::error::Error<T>>;

#[derive(Debug)]
struct Fish {
    age: u32,
}

impl Fish {
    fn born() -> Self {
        Self { age: 8 }
    }

    async fn sim_day(mut self) -> (Self, Option<Self>) {
        if self.age == 0 {
            self.age = 6;
            (self, Some(Fish::born()))
        } else {
            self.age -= 1;
            (self, None)
        }
    }
}

struct School {
    day: u32,
    fish: Vec<Fish>,
}

impl std::fmt::Debug for School {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fish_string = self
            .fish
            .iter()
            .map(|f| f.age.to_string())
            .intersperse(','.to_string())
            .collect::<String>();
        if self.day == 0 {
            write!(f, "Initial state: {}", fish_string)
        } else {
            write!(f, "After {:>2} days: {}", self.day, fish_string)
        }
    }
}

impl School {
    fn new(fish: &[u32]) -> Self {
        Self {
            day: 0,
            fish: fish.into_iter().map(|i| Fish { age: *i }).collect(),
        }
    }

    async fn sim_day(mut self) -> Self {
        self.day += 1;

        let capacity = self.fish.len() + (self.fish.len() / 6);
        self.fish = join_all(self.fish.into_iter().map(|f| f.sim_day()))
            .await
            .into_iter()
            .fold(
                Vec::with_capacity(capacity),
                |mut fs, (daddy_fish, maybe_baby)| {
                    fs.push(daddy_fish);
                    if let Some(baby_fish) = maybe_baby {
                        fs.push(baby_fish);
                    }
                    fs
                },
            );

        self
    }
}

pub async fn simulate_fish(data: &str, days: u32) -> Result<usize, Error<&str>> {
    let (_, fish) = separated_list1(char(','), map_res(digit1, str::parse::<u32>))(data)?;
    let mut school = School::new(&fish);
    for _ in 0..days {
        school = school.sim_day().await;
    }
    Ok(school.fish.len())
}

#[cfg(test)]
mod tests {
    extern crate test;

    use async_std::task::block_on;
    use futures::FutureExt;
    use test::Bencher;

    use super::*;

    const DATA: &str = "3,4,3,1,2";

    #[test]
    fn test_simulate_fish() -> Result<(), Error<&'static str>> {
        let dataset = [("small", 18, 26), ("big", 80, 5_934)];

        for (tag, days, expected) in dataset {
            block_on(simulate_fish(DATA, days).then(|result| async move {
                if let Ok(actual) = result {
                    println!("{}", tag);
                    assert_eq!(expected, actual);
                }
                result
            }))?;
        }
        Ok(())
    }

    #[bench]
    fn bench_school_sim_day(b: &mut Bencher) {
        let data = &DATA
            .split(',')
            .map(str::parse)
            .filter_map(Result::ok)
            .collect::<Vec<_>>()[..];
        b.iter(|| block_on(School::new(data).sim_day()))
    }

    #[bench]
    fn benchmark_simulated_fish_1_day(b: &mut Bencher) {
        b.iter(|| block_on(simulate_fish(DATA, 1)));
    }

    #[bench]
    fn benchmark_simulated_fish_10_days(b: &mut Bencher) {
        b.iter(|| block_on(simulate_fish(DATA, 10)));
    }
    #[bench]
    fn benchmark_simulated_fish_100_days(b: &mut Bencher) {
        b.iter(|| block_on(simulate_fish(DATA, 100)));
    }
}
