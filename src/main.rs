use std::io::{stdin, stdout, Write};

fn main() {
    // Read stdin
    let mut input = String::new();
    print!("Enter number of digits to test (e.g. 10_, 2d, 3c, 44k, 55M): ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    let mut min_multiplicity_log = String::new();
    print!("Enter the minimum multiplicity to break at (e.g. 2, 9, 111): ");
    stdout().flush().unwrap();
    stdin().read_line(&mut min_multiplicity_log).unwrap();
    let min_multiplicity_log_num = min_multiplicity_log.trim().parse().unwrap();
    let inp = input.clone();
    let inp = inp.trim();

    let num_digits = parse_num_digits(inp.to_string());
    let twos = "2".repeat(num_digits);
    let start = twos.parse::<usize>().unwrap();
    println!("{}", start);
    let end = usize::MAX - 1;
    let max_till_alloc = 22222222;
    let mut highest_checked = start + max_till_alloc;
    while highest_checked < end {
        let num_vec = create_number_strings(highest_checked, highest_checked + max_till_alloc);
        println!("\n🔵 Starting test...");
        let len_of_vec = num_vec.len();
        let mut c = 0;
        for num in num_vec {
            // Print progress every 2%
            if c % (len_of_vec / 50) == 0 {
                print!("\r🟡 {:?}%", c * 100 / len_of_vec);
                stdout().flush().unwrap();
            }
            let num_steps = multiplicity_persistence(&num);
            if num_steps >= min_multiplicity_log_num {
                println!("\n🔵 Multiplicity = {}, Number = {}", num_steps, num);
                break;
            }
            if c == len_of_vec - 1 {
                println!(
                    "\n🔴 No numbers found with multiplicity > {}\n",
                    min_multiplicity_log_num
                );
            }
            c += 1;
        }
        highest_checked += max_till_alloc;
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
/// Does not include the digits 0, 1, or 5
/// Numbers are sorted in ascending order
fn create_number_strings(start: usize, end: usize) -> Vec<String> {
    // Currently, this over-allocates the vector.
    // Need to confirm if it is correct to use:
    // 749_usize.pow((digits - 3).try_into().unwrap())
    let total_num = end - start;
    println!("\n🔵 Allocating memory for {}...", total_num);
    let mut num_vec: Vec<String> = Vec::with_capacity(total_num);

    let mut i = 0;
    let percent = (end - start) / 50;
    for num in start..=end {
        // Print progress every 2%
        if i % percent == 0 {
            print!("\r🟡 {:?}%", i * 100 / total_num);
            stdout().flush().unwrap();
        }
        let num_str = num.to_string();
        i += 1;
        if num_str.contains('5') || num_str.contains('1') || num_str.contains('0') {
            continue;
        }
        num_vec.push(num_str);
    }
    println!("\n🔵 Memory allocated and used.");
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
