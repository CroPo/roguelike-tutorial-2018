use rand::prelude::*;
use std::collections::HashMap;

pub fn random_choice_index(chances: Vec<i32>) -> usize {
    let mut rng = thread_rng();
    let random_chance = rng.gen_range(1, chances.iter().sum());



    let mut running_sum = 0;
    let mut choice = 0;

    for chance in chances {
        running_sum += chance;
        if random_chance <= running_sum {
            break
        }
        choice+=1;
    }
    choice
}