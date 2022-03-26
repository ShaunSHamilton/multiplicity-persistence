use std::io::{stdin, stdout, Write};

fn main() {
    // Read stdin
    let mut input = String::new();
    print!("Enter number of digits to test (e.g. 10_, 2d, 3c, 44k, 55M): ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    let mut min_multiplicity_log = String::new();
    print!("Enter the minimum multiplicity to log the number (e.g. 2, 3, 9, 11): ");
    stdout().flush().unwrap();
    stdin().read_line(&mut min_multiplicity_log).unwrap();
    let min_multiplicity_log_num = min_multiplicity_log.trim().parse().unwrap();
    let inp = input.clone();
    let inp = inp.trim();

    let num_digits = parse_num_digits(inp.to_string());
    if num_digits >= 3 {
        println!(
            "🟠 About to test {:?} numbers...",
            749_usize.pow((num_digits - 3).try_into().unwrap())
        );
    }
    let num_vec = create_number_strings(num_digits);
    for num in num_vec {
        let num_steps = multiplicity_persistence(&num);
        if num_steps > min_multiplicity_log_num {
            println!("🔵 Multiplicity = {}, Number = {}", num_steps, num);
            break;
        }
    }
}

fn multiplicity_persistence(strn: &String) -> usize {
    let mut multiplicity = 0;
    let mut string_rep = strn.clone();
    while string_rep.len() > 1 {
        let num: u32 = string_rep
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .reduce(|a, b| {
                let m = a * b;
                m
            })
            .unwrap();
        string_rep = num.to_string();
        multiplicity += 1;
    }
    multiplicity
}

/// Vec of all numbers with `digits` digits
/// Does not include the numbers 1 or 5
/// Numbers are sorted in ascending order
/// Numbers are split into groups of 19 digits
fn create_number_strings(digits: usize) -> Vec<String> {
    let mut num_vec: Vec<String> = Vec::with_capacity(10_usize.pow(digits as u32));
    let twos = "2".repeat(digits);
    let nines = "9".repeat(digits);
    let start = twos.parse::<usize>().unwrap();
    let end = nines.parse::<usize>().unwrap();
    for num in start..=end {
        let num_str = num.to_string();
        if num_str.contains('5') || num_str.contains('1') || num_str.contains('0') {
            continue;
        }
        num_vec.push(num_str);
    }
    num_vec
}

fn parse_num_digits(mut inp: String) -> usize {
    let mult = inp.pop().unwrap();
    let num = inp.parse::<usize>().unwrap();
    match mult {
        'd' => num * 10,
        'c' => num * 100,
        'k' => num * 1000,
        'M' => num * 1000000,
        'G' => num * 1000000000,
        'T' => num * 1000000000000,
        'P' => num * 1000000000000000,
        'E' => num * 1000000000000000000,
        _ => num,
    }
}
