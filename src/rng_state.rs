#[derive(Eq,Copy,Clone)]
pub struct RngState {
    seed: u32,
    initial_state: u32
}

impl RngState {
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            initial_state: RngState::seed_to_initial_state(seed)
        }
    }

    pub fn seed(&self) -> u32 {
        self.seed
    }

    fn seed_to_initial_state(seed: u32) -> u32 {
        seed.wrapping_mul(0x343fd).wrapping_add(0x269ec3) >> 0x10
    }

    pub fn find_good_seed(syncing_state: RngState, desired_seed: u32 ) -> u32 {
        for test_seed in desired_seed.. {
            if syncing_state == test_seed.into() {
                return test_seed
            }
        };
        unreachable!()
    }
}

impl From<u32> for RngState {
    fn from(seed: u32) -> Self {
        RngState::new(seed)
    }
}

impl PartialEq for RngState {
    fn eq(&self, other: &Self) -> bool {
        // if the seeds are the same the states are the same. if not, check the other state anyway
        self.seed == other.seed 
           || self.initial_state == other.initial_state
    }
}