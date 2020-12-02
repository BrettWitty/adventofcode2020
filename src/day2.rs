use itertools::Itertools;
use rayon::prelude::*;

pub struct Policy {
    min_count: usize,
    max_count: usize,
    letter: char,
    password: String
}

impl Policy {

    fn new(min_count: usize, max_count: usize, letter: char, password: String) -> Self {
        Policy{min_count, max_count, letter, password}
    }

    fn is_valid(&self) -> bool {

        let letter_count = self.password.chars().filter(|&c| c == self.letter).count();

        (self.min_count <= letter_count) & (self.max_count >= letter_count)

    }

    fn is_valid_v2(&self) -> bool {

        // Rust doesn't have trivial string indexing, so we have to do this.

        let pos1 = self.letter == self.password.chars().nth(self.min_count-1).unwrap();
        let pos2 = self.letter == self.password.chars().nth(self.max_count-1).unwrap();

        // We can be at one position or the other, but not both, so XOR
        pos1 ^ pos2

    }

}

#[aoc_generator(day2)]
pub fn generate_part1(input: &str) -> Vec<Policy> {

    let mut policies = Vec::new();

    for (a, b, c) in input.lines().map( |l| { l.trim().split(" ").collect_tuple().unwrap() }) {

        let (min,max) = a.split("-").map( |w| w.parse().unwrap() ).collect_tuple().unwrap();

        let policy = Policy::new(min,max, b.chars().nth(0).unwrap(), c.to_string());

        policies.push(policy);

    }
        
    policies

}

#[aoc(day2,part1,direct)]
pub fn solve_part1(input: &Vec<Policy> ) -> usize {

    input.iter().filter( |&p| p.is_valid() ).count()

}

#[aoc(day2,part2,direct)]
pub fn solve_part2(input: &Vec<Policy> ) -> usize {

    input.iter().filter( |&p| p.is_valid_v2() ).count()

}

#[aoc(day2,part1,rayon)]
pub fn solve_part1_parallel(input: &Vec<Policy> ) -> usize {

    input.par_iter().filter( |&p| p.is_valid() ).count()

}

#[aoc(day2,part2,rayon)]
pub fn solve_part2_parallel(input: &Vec<Policy> ) -> usize {

    input.par_iter().filter( |&p| p.is_valid_v2() ).count()

}

