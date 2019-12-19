mod parser;
use self::parser::formulae;

#[aoc_generator(day14)]
pub fn requirements(input: &str) -> Vec<(&str, usize), Vec<(&str, usize)>> {
    formulae(input).unwrap().1
}

#[aoc(day14, part1)]
pub fn fuel_requirements(input: &[((&str, usize), Vec<(&str, usize)>)]) -> u64 {
    for i in input {
        print!("{} {} =>", i.0.0, i.0.1);
        for r in i.1 {
            print!(" {}{}", r.0, r.1);
        }
        println!();
    }
    3
}
