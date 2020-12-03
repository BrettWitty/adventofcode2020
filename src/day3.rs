use std::collections::HashSet;
use itertools::Itertools;

pub struct Map {

    height: usize,
    width: usize,
    terrain: Vec< HashSet<usize> >

}

impl Map {

    fn check_slope(&self, x: usize, y: usize) -> usize {

        let mut j : usize = 0;
        let mut trees = 0;

        // Step down the slope
        for i in (0..self.height).step_by(x) {

            // Check if (i,j) is a tree
            if self.terrain[i].contains(&j) {
                trees += 1;
            }

            // Adjust j
            j += y;
            j %= self.width;

        }

        trees

    }

}

#[aoc_generator(day3)]
pub fn generate_part1( input: &str ) -> Map {

    const TREE: char = '#';

    let lines : Vec<String> = input.lines().map(|l| l.to_string()).collect();

    let width = lines[0].len();
    let height = lines.len();

    // This is the trickiest line:
    // For each row, then count how many characters are trees in that row and record by position
    // This gives us ultra-fast lookup
    let terrain: Vec< HashSet<usize> > =  lines.iter().map( |s| s.chars().positions( |l| l == TREE ).collect() ).collect();

    Map{height, width, terrain}

}

#[aoc(day3,part1)]
pub fn solve_part1( input: &Map ) -> usize {

    input.check_slope(1,3)

}

#[aoc(day3,part2)]
pub fn solve_part2( input: &Map ) -> usize {

    let slopes = [ (1,1), (1,3), (1,5), (1,7), (2,1) ];

    slopes.iter().map( |x| input.check_slope(x.0, x.1)).product1().unwrap()

}


//----------------------------------------
// Fast version?

pub struct MapFast {

    height: usize,
    width: usize,
    terrain: Vec< u64 >

}

impl MapFast {

    fn check_slope(&self, x: usize, y: usize) -> usize {

        let mut j : usize = 0;
        let mut trees = 0;

        // Step down the slope
        for i in (0..self.height).step_by(x) {

            // Check if (i,j) is a tree
            if self.terrain[i] & (1 << j) > 0 {
                trees += 1;
            }

            // Adjust j
            j += y;
            j %= self.width;

        }

        trees

    }

}


#[aoc_generator(day3,part1,fast)]
pub fn generate_part1_fast( input: &str ) -> MapFast {

    const TREE: char = '#';

    let lines : Vec<String> = input.lines().map(|l| l.to_string()).collect();

    let width = lines[0].len();
    let height = lines.len();

    // This is the trickiest line:
    // For each row, then count how many characters are trees in that row and record by position
    // This gives us ultra-fast lookup
    let terrain: Vec< u64 > =  lines.iter()
        .map( |s| s.chars()
               .positions( |l| l == TREE )
               .map( |i| (1 << i) as u64)
               .fold(0_u64, |acc,x| acc | x))
        .collect();

    MapFast{height, width, terrain}

}
#[aoc_generator(day3,part2,fast)]
pub fn generate_part2_fast( input: &str ) -> MapFast {

    const TREE: char = '#';

    let lines : Vec<String> = input.lines().map(|l| l.to_string()).collect();

    let width = lines[0].len();
    let height = lines.len();

    // This is the trickiest line:
    // For each row, then count how many characters are trees in that row and record by position
    // This gives us ultra-fast lookup
    let terrain: Vec< u64 > =  lines.iter()
        .map( |s| s.chars()
               .positions( |l| l == TREE )
               .map( |i| (1 << i) as u64)
               .fold(0_u64, |acc,x| acc | x))
        .collect();

    MapFast{height, width, terrain}

}

#[aoc(day3,part1, fast)]
pub fn solve_part1_fast( input: &MapFast ) -> usize {

    input.check_slope(1,3)

}

#[aoc(day3,part2, fast)]
pub fn solve_part2_fast( input: &MapFast ) -> usize {

    let slopes = [ (1,1), (1,3), (1,5), (1,7), (2,1) ];

    slopes.iter().map( |x| input.check_slope(x.0, x.1)).product1().unwrap()

}
