use std::collections::HashMap;
use std::hash::Hash;

use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list1;

type Error<T> = nom::Err<nom::error::Error<T>>;

fn insert_or_update_count<K>(mut map: HashMap<K, u64>, key: K, count: u64) -> HashMap<K, u64>
where
    K: Eq + Hash,
{
    map.entry(key).and_modify(|c| *c += count).or_insert(count);
    map
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Fish {
    age: u32,
}

impl Fish {
    fn born() -> Self {
        Self { age: 8 }
    }

    fn sim_day(mut self) -> (Self, Option<Self>) {
        if self.age == 0 {
            self.age = 6;
            (self, Some(Fish::born()))
        } else {
            self.age -= 1;
            (self, None)
        }
    }
}

#[derive(Debug)]
struct School {
    day: u32,
    fish: HashMap<Fish, u64>,
}

impl School {
    fn new(fish: &[u32]) -> Self {
        Self {
            day: 0,
            fish: fish
                .iter()
                .map(|i| Fish { age: *i })
                .fold(HashMap::with_capacity(9), |hm, f| {
                    insert_or_update_count(hm, f, 1)
                }),
        }
    }

    fn sim_day(mut self) -> Self {
        self.day += 1;
        self.fish = self
            .fish
            .into_iter()
            .fold(HashMap::with_capacity(9), |mut fs, (f, count)| {
                let (daddy_fish, maybe_baby) = f.sim_day();

                fs = insert_or_update_count(fs, daddy_fish, count);

                if let Some(baby_fish) = maybe_baby {
                    fs = insert_or_update_count(fs, baby_fish, count);
                }

                fs
            });

        self
    }

    fn total_fish(&self) -> u64 {
        self.fish.values().sum()
    }
}

pub fn simulate_fish(data: &str, days: u32) -> Result<u64, Error<&str>> {
    let (_, fish) = separated_list1(char(','), map_res(digit1, str::parse::<u32>))(data)?;
    let mut school = School::new(&fish);
    for _ in 0..days {
        school = school.sim_day();
    }
    Ok(school.total_fish())
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    const DATA: &str = "3,4,3,1,2";

    #[test]
    fn test_simulate_fish() -> Result<(), Error<&'static str>> {
        let dataset = [
            ("small", 18, 26),
            ("big", 80, 5_934),
            ("huge", 256, 26_984_457_539),
        ];

        for (tag, days, expected) in dataset {
            println!("{tag}");
            assert_eq!(expected, simulate_fish(DATA, days)?);
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
        b.iter(|| School::new(data).sim_day())
    }

    #[bench]
    fn benchmark_simulated_fish_1_day(b: &mut Bencher) {
        b.iter(|| simulate_fish(DATA, 1));
    }

    #[bench]
    fn benchmark_simulated_fish_10_days(b: &mut Bencher) {
        b.iter(|| simulate_fish(DATA, 10));
    }
    #[bench]
    fn benchmark_simulated_fish_100_days(b: &mut Bencher) {
        b.iter(|| simulate_fish(DATA, 100));
    }
}
