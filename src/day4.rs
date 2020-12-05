use itertools::Itertools;
use std::collections::HashMap;

const REQUIRED : [&str; 7] = [ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" ];

#[derive(Debug)]
pub struct Passport {

    fields: HashMap<String,String>
    
}

impl Passport {

    fn new(input: &str ) -> Self {

        let fields = input.to_owned().split_whitespace().map( |s| s.split(':').map_into::<String>().collect_tuple().unwrap() ).collect();

        Self { fields }

    }

    fn is_valid(&self) -> bool {

        REQUIRED.iter().all( |&x| self.fields.contains_key(x))

    }

    fn is_valid_v2(&self) -> bool {

        if self.is_valid() {

            for (k,v) in self.fields.iter() {

                let val = match k.as_str() {

                    "byr" => self.byr_valid(v),
                    "iyr" => self.iyr_valid(v),
                    "eyr" => self.eyr_valid(v),
                    "hgt" => self.hgt_valid(v),
                    "hcl" => self.hcl_valid(v),
                    "ecl" => self.ecl_valid(v),
                    "pid" => self.pid_valid(v),
                    "cid" => true,
                    _ => false

                };

                if !val {

                    return false;
                }

            }

            return true;

        }
        else {
            return false;
        }

    }

    fn byr_valid(&self, val: &str) -> bool {

        if let Ok(year) = val.parse::<usize>() {
            return (1920 <= year) && (year <= 2002);
        }

        return false;

    }

    fn iyr_valid(&self, val: &str) -> bool {

        if let Ok(year) = val.parse::<usize>() {
            return (2010 <= year) && (year <= 2020);
        }

        return false;

    }

    fn eyr_valid(&self, val: &str) -> bool {

        if let Ok(year) = val.parse::<usize>() {
            return (2020 <= year) && (year <= 2030);
        }

        return false;

    }

    fn hgt_valid(&self, val: &str) -> bool {

        if val.ends_with("cm") {

            let num = val.strip_suffix("cm").unwrap().parse::<usize>().unwrap();

            return (150 <= num) && (num <= 193);
        }
        else if val.ends_with("in") {
            let num = val.strip_suffix("in").unwrap().parse::<usize>().unwrap();

            return (59 <= num) && (num <= 76);

        }
        else {
            return false;
        }

    }

    fn hcl_valid(&self, val: &str) -> bool {

        return match val.strip_prefix("#") {
            Some(val) => val.chars().all( |c| "1234567890abcdef".contains(c) ) && (val.len() == 6),
            None => false
        }

    }

    fn ecl_valid(&self, val: &str) -> bool {

        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val)

    }

    fn pid_valid(&self, val: &str) -> bool {

        val.chars().all( |c| "1234567890".contains(c) ) && val.len() == 9
    }

}


#[aoc_generator(day4)]
pub fn generate_part1( input: &str ) -> Vec<Passport> {

    input.split("\n\n")
        .map( |s| Passport::new(&s) ).collect()

}

#[aoc(day4,part1)]
pub fn part1( input: &Vec<Passport> ) -> usize {

    input.iter().filter( |s| s.is_valid() ).count()

}

#[aoc(day4,part2)]
pub fn part2( input: &Vec<Passport> ) -> usize {

    input.iter().filter( |s| s.is_valid_v2() ).count()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_passports() {

        let val = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let passports: Vec<Passport> = val.split("\n\n").map( |s| Passport::new(&s) ).collect();

        assert_eq!( passports.len(), 4 );
        assert_eq!( passports.iter().filter( |s| s.is_valid_v2() ).count(), 4);

    }

    #[test]
    fn invalid_passports() {

        let val = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        let passports : Vec<Passport> = val.split("\n\n").map( |s| Passport::new(&s) ).collect();

        assert_eq!( passports.len(), 4 );
        assert_eq!( passports.iter().filter( |s| s.is_valid_v2() ).count(), 0);

    }
}
