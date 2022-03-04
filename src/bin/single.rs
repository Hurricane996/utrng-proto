use utrng::utils::get_int;
use utrng::rng_state::RngState;

fn main() {
    let syncing_seed = get_int("Syncing start time: ");
    let desired_seed = get_int("Desired start time: ");

    let best_seed = RngState::find_good_seed(syncing_seed.into(), desired_seed);

    println!("{best_seed} syncs, add {} us", best_seed - desired_seed);
    
}