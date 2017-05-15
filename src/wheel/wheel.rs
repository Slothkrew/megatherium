extern crate rand;
use self::rand::{thread_rng, Rng};

pub fn spin() -> String {
    let mut rng = thread_rng();
    let punishments = vec! {
        "death by the crazy 88s",
        "death by surprise jihad",
        "death by cantrymen",
        "forced to program in ArnoldC for a living",
        "dot|not hits you with a wrench",
        format!("gulag {} years", rng.gen_range(0, 1_000_000))
    };    
    let i = rng.gen_range(0, punishments.len());
    punishments[i].to_string()
}
