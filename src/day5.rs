use std::collections::HashSet;

pub struct Seat {
    row: usize,
    col: usize
}

impl Seat {
    fn get_id(&self) -> usize {
        self.row*8 + self.col
    }
}

fn decode(text: &str, one: char, _zero: char ) -> usize {

    // Big-endian usize decoding
    let mut c : usize = 0;

    for letter in text.chars() {

        let mut n = 0;

        if letter == one {
            n = 1;
        }

        c = 2*c + n;

    }

    return c;

}

impl From<(&str,&str)> for Seat {

    fn from(text: (&str,&str)) -> Seat {

        Seat{ row: decode(text.0, 'B', 'F'), col: decode(text.1, 'R', 'L') }

    }
    
}

#[aoc_generator(day5)]
pub fn generate(input: &str ) -> Vec<Seat> {

    input.lines().map( |s| s.split_at(7).into() ).collect()

}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<Seat>) -> usize {

    input.iter().map( |s| s.get_id() ).max().unwrap()

}

#[aoc(day5, part2)]
pub fn part2(input: &Vec<Seat>) -> usize {

    let ids : HashSet<usize> = input.iter().map( |s| s.get_id() ).collect();

    for id in (1..1023) {

        if !ids.contains(&id) {

            if ids.contains(&(id -1)) && ids.contains(&(id +1)) {
                return id;
            }
        }

    }

    return 0;

}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {

        let s = "BFFFBBFRRR";

        let seat : Seat = s.split_at(7).into();

        assert_eq!(seat.row, 70);
        assert_eq!(seat.col, 7);
        assert_eq!(seat.get_id(), 567);

    }

    #[test]
    fn test2() {

        let s = "FFFBBBFRRR";

        let seat : Seat = s.split_at(7).into();

        assert_eq!(seat.row, 14);
        assert_eq!(seat.col, 7);
        assert_eq!(seat.get_id(), 119);

    }

    #[test]
    fn test3() {

        let s = "BBFFBBFRLL";

        let seat : Seat = s.split_at(7).into();

        assert_eq!(seat.row, 102);
        assert_eq!(seat.col, 4);
        assert_eq!(seat.get_id(), 820);

    }

    


}
