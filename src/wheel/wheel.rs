extern crate rand;
use self::rand::{thread_rng, Rng};

pub fn spin() -> String {
    let mut rng = thread_rng();
    let punishments = vec! {
        format!("death by the crazy 88s"),
        format!("death by surprise jihad"),
        format!("death by cantrymen"),
        format!("forced to program in ArnoldC for a living"),
        format!("dot|not hits you with a wrench"),
        format!("gulag {} years", rng.gen_range(0, 1_000_000))
    };
    let i = rng.gen_range(0, punishments.len());
    punishments[i].to_string()
}

pub fn about() -> String {
    "glorious wheel best wheel".to_string()
}

pub fn help() -> String {
        "```\
        ***********************************************\n\
        | wheel: best wheel utility                   |\n\
        |---------------------------------------------|\n\
        | !wheel       | fair sentence just sentence  |\n\
        | !wheel about | learn wheel understand wheel |\n\
        ***********************************************\n\
        ```".to_string()
}