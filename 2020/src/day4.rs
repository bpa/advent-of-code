use regex::Regex;
use std::collections::HashSet;

#[aoc(day4, part1)]
pub fn valid_passports(input: &str) -> usize {
    let required: HashSet<String> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|&f| String::from(f))
        .collect();
    let mut existing = HashSet::with_capacity(16);
    let mut count = 0;
    for line in input.lines() {
        if line.is_empty() {
            if required.is_subset(&existing) {
                count += 1;
            }
            existing.clear();
        } else {
            for field in line
                .split_ascii_whitespace()
                .map(|f| f.split(":").next().unwrap())
            {
                existing.insert(String::from(field));
            }
        }
    }
    if required.is_subset(&existing) {
        count += 1;
    }
    count
}

#[aoc(day4, part2)]
pub fn validated_passports(input: &str) -> usize {
    let height_re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let hair_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let eye_re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    let pid_re = Regex::new(r"^\d{9}$").unwrap();
    let required: HashSet<String> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|&f| String::from(f))
        .collect();
    let mut existing = HashSet::with_capacity(16);
    let mut count = 0;
    for line in input.lines() {
        if line.is_empty() {
            if required.is_subset(&existing) {
                count += 1;
            }
            existing.clear();
        } else {
            for mut field in line.split_ascii_whitespace().map(|f| f.split(":")) {
                let name = field.next().unwrap();
                let value = field.next().unwrap();
                if match name {
                    "byr" => {
                        if let Ok(year) = value.parse::<usize>() {
                            year >= 1920 && year <= 2002
                        } else {
                            false
                        }
                    }
                    "iyr" => {
                        if let Ok(year) = value.parse::<usize>() {
                            year >= 2010 && year <= 2020
                        } else {
                            false
                        }
                    }
                    "eyr" => {
                        if let Ok(year) = value.parse::<usize>() {
                            year >= 2020 && year <= 2030
                        } else {
                            false
                        }
                    }
                    "hgt" => {
                        if let Some(cap) = height_re.captures(value) {
                            let h = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
                            match cap.get(2).unwrap().as_str() {
                                "cm" => h >= 150 && h <= 193,
                                "in" => h >= 59 && h <= 76,
                                _ => false,
                            }
                        } else {
                            false
                        }
                    }
                    "hcl" => hair_re.is_match(value),
                    "ecl" => eye_re.is_match(value),
                    "pid" => pid_re.is_match(value),
                    _ => true,
                } {
                    existing.insert(String::from(name));
                }
            }
        }
    }
    if required.is_subset(&existing) {
        count += 1;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            2,
            valid_passports(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929
      
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in"
            )
        )
    }

    #[test]
    fn part2() {
        assert_eq!(
            0,
            validated_passports(
                "eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946

        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007"
            )
        );
        assert_eq!(
            4,
            validated_passports(
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f

        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022

        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        "
            )
        );
    }
}
