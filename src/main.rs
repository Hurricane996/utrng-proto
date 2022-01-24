
use std::io::{
    stdin, stdout, Write
};

use std::time::Instant;


fn get_int(question: &str) -> u32 {
    loop {
        print!("{}", question);
        let mut s = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Could not read from stdin");

        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        match s.parse::<u32>() {
            Ok(res) => {
                return res;
            },
            Err(_) => {
                print!("Not a valid int!");
                continue;
            },
        }
    }
}

fn to_state(seed: u32) -> u32 {
    seed.wrapping_mul(0x343fd).wrapping_add(0x269ec3) >> 0x10
}

fn test(initial_seed: u32, target_state: u32) -> u32 {
    for count in 0.. {
        let test_state = to_state(initial_seed + count);

        if test_state == target_state {
            return count
        }
    };
    unreachable!();
}


fn main() {
    let target_seed = get_int("Target (syncing) system time: ");
    let initial_seed = get_int("Desired (non-syncing) system time: ");


    println!("Calculating..");
    let start_time = Instant::now();

    let target_state = to_state(target_seed);

    let us_to_add = test(initial_seed, target_state);

    println!("Found seed {} (add {} us) after {:.2?} ", initial_seed + us_to_add, us_to_add, start_time.elapsed()  );
}


