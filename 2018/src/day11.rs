use std::cmp::max;

#[aoc_generator(day11)]
fn read_serial(input: &str) -> isize {
    input.parse::<isize>().unwrap()
}

#[aoc(day11, part1)]
fn global_maximum(serial: &isize) -> String {
    let mut x_max = 0;
    let mut y_max = 0;
    let mut max_power = 0;
    let mut cells = [0; 900];
    populate_row_sums(&mut cells, *serial, 1);
    populate_row_sums(&mut cells, *serial, 2);
    for y in 3..=300 {
        populate_row_sums(&mut cells, *serial, y);
        for x in 0..298 {
            let value = cells[x] + cells[300 + x] + cells[600 + x];
            if value > max_power {
                max_power = value;
                x_max = x + 1;
                y_max = y - 2;
            }
        }
    }
    format!("{},{}", x_max, y_max)
}

#[inline(always)]
fn power_level(x: isize, y: isize, serial: isize) -> isize {
    let rack_id = x + 10;
    let power = (rack_id * y + serial) * rack_id;
    ((power / 100) % 10) as isize - 5
}

fn populate_row_sums(cells: &mut [isize; 900], serial: isize, y: isize) {
    let start: isize = (y % 3) * 300 - 3;
    let mut trailing = [0; 3];
    trailing[1] = power_level(1, y, serial);
    trailing[2] = power_level(2, y, serial);
    let mut value = trailing[1] + trailing[2];
    for x in 3..=300isize {
        let i = (x % 3) as usize;
        let next = power_level(x, y, serial);
        value = value - trailing[i] + next;
        trailing[i] = next;
        cells[(start + x) as usize] = value;
    }
}

#[aoc(day11, part2)]
fn using_area_lookup(serial: &isize) -> String {
    let mut max_sum = isize::MIN;
    let (mut max_x, mut max_y, mut size) = (0, 0, 0);
    let lookup_table = create_lookup_table(*serial);
    for x in 1..298 {
        for y in 1..298 {
            for z in 1..(300 - max(x, y)) {
                let local_sum = lookup_table[x][y] + lookup_table[x + z][y + z]
                    - lookup_table[x][y + z]
                    - lookup_table[x + z][y];
                if local_sum > max_sum {
                    max_sum = local_sum;
                    max_x = x + 1;
                    max_y = y + 1;
                    size = z;
                }
            }
        }
    }
    format!("{},{},{}", max_x, max_y, size)
}

fn create_lookup_table(serial: isize) -> [[isize; 301]; 301] {
    let mut table = [[0; 301]; 301];
    for x in 1..=300 {
        for y in 1..=300 {
            table[x][y] = table[x - 1][y] + power_level(x as isize, y as isize, serial);
        }
    }
    for x in 1..=300 {
        let col = &mut table[x];
        for y in 1..=300 {
            col[y] += col[y - 1];
        }
    }
    table
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn power_levels() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn part1() {
        assert_eq!("33,45", global_maximum(&18));
        assert_eq!("21,61", global_maximum(&42));
    }

    #[test]
    fn part2() {
        assert_eq!("90,269,16", using_area_lookup(&18));
        assert_eq!("232,251,12", using_area_lookup(&42));
    }
}
