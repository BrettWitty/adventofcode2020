use itertools::Itertools;
use std::collections::HashSet;
use std::convert::From;

#[aoc_generator(day6,part1)]
pub fn generate(input: &str ) -> Vec<HashSet<char>> {

    input.split("\n\n")
        .map( |group| group.chars().filter( |c| c.is_ascii_alphabetic() ).collect::<HashSet<char>>())
        .collect()

}

#[aoc(day6,part1)]
pub fn part1( input: &Vec<HashSet<char>> ) -> usize {
    input.iter().map( |h| h.len() ).sum()
}



#[aoc_generator(day6,part2)]
pub fn generate2(input: &str ) -> Vec<HashSet<char>> {

    let mut result = Vec::new();
    
    for group in input.split("\n\n") {

        let mut record = HashSet::new();
        let mut made_record = false;

        for line in group.lines() {
            let letters: HashSet<char> = line.chars().filter( |c| c.is_ascii_alphabetic()).collect();

            if made_record {
                record = record.intersection(&letters).map(|c| *c).collect();
            }
            else {
                record = letters.clone();
                made_record = true;
            }

            
        }

        result.push(record);

    }

    result

}

#[aoc(day6,part2)]
pub fn part2( input: &Vec<HashSet<char>> ) -> usize {
    input.iter().map( |h| h.len() ).sum()
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {

        let s = "abc

a
b
c

ab
ac

a
a
a
a

b";

        let vals = generate2(s);
        let counts : Vec<usize> = vals.iter().map(|h| h.len()).collect();

        dbg!(vals);
        assert_eq!(vec![3,0,1,1,1], counts);
    }

}
