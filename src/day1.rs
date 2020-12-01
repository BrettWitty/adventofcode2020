use itertools::Itertools;

#[aoc_generator(day1)]
pub fn generate_part1(input: &str) -> Vec<u32> {

    input
        .lines()
        .map(|l| {
            l.trim().parse().unwrap()
        } ).collect()

}

#[aoc(day1,part1)]
pub fn solve_part1(input: &[u32]) -> u32 {

    for v in input.iter().combinations(2) {
        let a = v[0];
        let b = v[1];
        if a+b == 2020 {
            return a*b;
        }
    }

    0
        
}

#[aoc(day1,part2)]
pub fn solve_part2(input: &[u32]) -> u32 {

    for v in input.iter().combinations(3) {
        if v[0] + v[1] + v[2] == 2020 {
            return v[0] * v[1] * v[2];
        }
    }

    0
        
}

