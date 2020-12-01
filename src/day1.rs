use itertools::Itertools;

#[aoc_generator(day1)]
pub fn generate_part1(input: &str) -> Vec<u32> {

    // Simply split the file into lines, trim and parse each as a u32, then return

    input
        .lines()
        .map(|l| {
            l.trim().parse().unwrap()
        } ).collect()

}

#[aoc(day1,part1,direct)]
pub fn solve_part1(input: &[u32]) -> u32 {

    // The main juice here is itertools' combinations
    // I tried a pure itertor version but it was way uglier and slower!

    for v in input.iter().combinations(2) {
        let a = v[0];
        let b = v[1];
        if a+b == 2020 {
            return a*b;
        }
    }

    0
        
}

#[aoc(day1,part2,direct)]
pub fn solve_part2(input: &[u32]) -> u32 {

    // I tried a pure itertor version but it was way uglier and slower!
    
    for v in input.iter().combinations(3) {
        if v[0] + v[1] + v[2] == 2020 {
            return v[0] * v[1] * v[2];
        }
    }

    0
        
}

pub struct OddsEvens {

    pub odds: Vec<u32>,
    pub evens: Vec<u32>
    
}

// Because 2020 is even, it's either the sum of two evens, or sum of
// two odds, so we get a cut-down by focussing on just one
#[aoc_generator(day1,part1,oddeven)]
pub fn generate_part1_oddeven(input: &str) -> OddsEvens {

    // Simply split the file into lines, trim and parse each as a u32, then return

    let v = input
        .lines()
        .map(|l| {
            l.trim().parse().unwrap()
        } ).partition( |n: &u32| n % 2 == 0);

    OddsEvens { odds: v.0, evens: v.1}

}

#[aoc_generator(day1,part2,oddeven)]
pub fn generate_part2_oddeven(input: &str) -> OddsEvens {

    // Simply split the file into lines, trim and parse each as a u32, then return

    let v = input
        .lines()
        .map(|l| {
            l.trim().parse().unwrap()
        } ).partition( |n: &u32| n % 2 == 0);

    OddsEvens { odds: v.0, evens: v.1}

}

#[aoc(day1,part1,oddeven)]
pub fn solve_part1_oddeven(input: &OddsEvens) -> u32 {

    let evens = input.evens.clone();
    let odds = input.odds.clone();

    // 2020 is even,
    // So we either need even + even
    // Or odd + odd
    // So reduces 1/2*n*(n-1) = 1/2 (n^2-n) comparisons to 1/2*n*(n/2-1) = 1/4 n^2 - 1/2n

    for v in evens.iter().combinations(2) {
        let a = v[0];
        let b = v[1];
        if a+b == 2020 {
            return a*b;
        }
    }

    for v in odds.iter().combinations(2) {
        let a = v[0];
        let b = v[1];
        if a+b == 2020 {
            return a*b;
        }
    }
    
    0
        
}

#[aoc(day1,part2,oddeven)]
pub fn solve_part2_oddeven(input: &OddsEvens) -> u32 {

    let evens = input.evens.clone();
    let odds = input.odds.clone();

    // Same trick except
    // Even = Even + Even + Even
    // or
    // Even = Even + Odd + Odd

    for v in evens.iter().combinations(3) {
        let a = v[0];
        let b = v[1];
        let c = v[2];
        if a+b+c == 2020 {
            return a*b;
        }
    }

    for c in evens.iter() {
        for v in odds.iter().combinations(2) {
            let a = v[0];
            let b = v[1];
            if a+b+c == 2020 {
                return a*b;
            }
        }
    }
    
    0
        
}
