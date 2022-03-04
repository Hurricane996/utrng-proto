use utrng::rng_state::RngState;
use utrng::utils::get_int;

use utrng::utils::get_yn;

struct RandomizeCall {
    syncing_state: RngState,
    desired_delta_seed: u32
}

fn main() {
    let initial_start_time = get_int("Current start time");

    let mut times = vec![RandomizeCall {
        syncing_state: initial_start_time.into(),
        desired_delta_seed: 0
    }];

    
    for i in 0.. {
        let syncing_state = get_int(&format!("Randomize call {i} syncing time: ")).into();
        let desired_delta_seed = get_int(&format!("Randomize call {i} desired time: "));

        times.push(RandomizeCall {
            syncing_state,
            desired_delta_seed
        });

        if !get_yn("More randomize call? (y/n)") {
            break
        }
    }

    // set up stopping code

    println!("Calculating... (press Ctrl-c when you're bored)");

    let mut best_us = u32::MAX;

    let start_rng = RngState::new(initial_start_time);

    for start_time in 0.. {
        if start_rng != initial_start_time.into()  {
            println!("Bad start time");
            continue
        }
        
        let mut total_us_to_add = 0;

        for call in &times {
            let desired_seed = start_time + call.desired_delta_seed;

            let syncing_seed = RngState::find_good_seed(call.syncing_state, desired_seed) ;
            
            total_us_to_add += syncing_seed - desired_seed;
        }

        println!("Start time {start_time} adds {total_us_to_add} us");

        if total_us_to_add < best_us {
            best_us = total_us_to_add;

            println!("Found new best: {total_us_to_add} total microseconds with start time {start_time}")
        }
    }
}


