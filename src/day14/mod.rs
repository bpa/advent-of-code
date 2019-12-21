mod parser;
use self::parser::formulae;
use std::collections::HashMap;

#[aoc_generator(day14)]
pub fn requirements(input: &str) -> Vec<((String, usize), Vec<(String, usize)>)> {
    formulae(input).unwrap().1
}

#[aoc(day14, part1)]
pub fn fuel_requirements(input: &[((String, usize), Vec<(String, usize)>)]) -> usize {
    let mut transformations = HashMap::new();
    for f in input {
        transformations.insert(&(f.0).0, f);
    }
    create(1, &String::from("FUEL"), &transformations)
}

#[aoc(day14, part2)]
pub fn one_trillion_ore(input: &[((String, usize), Vec<(String, usize)>)]) -> usize {
    let one_trillion: usize = 1_000_000_000_000;
    let mut transformations = HashMap::new();
    let fuel = &String::from("FUEL");
    for f in input {
        transformations.insert(&(f.0).0, f);
    }

    let mut ore = create(1, fuel, &transformations);
    let mut count = one_trillion / ore;
    while ore < one_trillion {
        count = count * 2;
        ore = create(count, fuel, &transformations);
    }

    let mut higher = count;
    let mut lower = count / 2;
    loop {
        if ore < one_trillion {
            lower = count;
        } else {
            higher = count;
        }
        count = lower + (higher - lower) / 2;
        if count == lower {
            break;
        }
        ore = create(count, fuel, &transformations);
    }
    count
}

fn create<'a, 'b>(
    quantity: usize,
    thing: &'b String,
    transformations: &'a HashMap<&'b String, &'b ((String, usize), Vec<(String, usize)>)>,
) -> usize {
    let mut have: HashMap<&String, usize> = HashMap::new();
    let mut need: Vec<(&String, usize)> = vec![(thing, quantity)];
    let mut ore: usize = 0;

    while !need.is_empty() {
        let chemical = need.pop().unwrap();
        if chemical.0 == "ORE" {
            ore += chemical.1;
            continue;
        }

        let mut existing: usize = *have.entry(&chemical.0).or_default();
        if existing < chemical.1 {
            let recipe = transformations.get(chemical.0).unwrap();
            let missing = chemical.1 - existing;
            let multiplier = if missing % (recipe.0).1 == 0 {
                missing / (recipe.0).1
            } else {
                missing / (recipe.0).1 + 1
            };
            let new_items = (recipe.0).1 * multiplier;
            existing += new_items;
            for e in &recipe.1 {
                need.push((&e.0, e.1 * multiplier));
            }
        }
        existing -= chemical.1;
        have.insert(&chemical.0, existing);
    }
    ore
}
