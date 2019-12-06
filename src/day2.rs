#[aoc(day2, part1)]
pub fn run_12_2(input: &str) -> isize {
    let mut mem = read_memory(input);
    mem[1] = 12;
    mem[2] = 2;

    run(mem)
}

#[aoc(day2, part2)]
pub fn find_19690720(input: &str) -> String {
    let mem = read_memory(input);

    for noun in 0..mem.len() {
        for verb in 0..mem.len() {
            let mut code = mem.clone();
            code[1] = noun as isize;
            code[2] = verb as isize;

            if run(code) == 19690720 {
                return answer(noun, verb);
            }
        }
    }

    panic!("No solution found");
}

fn read_memory(input: &str) -> Vec<isize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

fn run(mut code: Vec<isize>) -> isize {
    let mut i: usize = 0;

    while i < code.len() {
        let op = code[i];
        if op == 99 {
            break;
        }

        let noun = code[i + 1] as usize;
        let verb = code[i + 2] as usize;
        let addr = code[i + 3] as usize;
        match op {
            1 => code[addr] = code[noun] + code[verb],
            2 => code[addr] = code[noun] * code[verb],
            _ => panic!("Invalid instruction found"),
        }
        i += 4;
    }

    code[0]
}

fn answer(noun: usize, verb: usize) -> String {
    let left = noun.to_string();
    let right = verb.to_string();
    format!("{}{:0width$}", left, right, width = left.len())
}
