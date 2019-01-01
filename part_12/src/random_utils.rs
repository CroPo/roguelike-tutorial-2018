use rand::prelude::*;
use std::collections::HashMap;
use std::borrow::Cow;

/// Returns a randomly selected index from a weighted list
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

/// Returns a value from a list of values by dungeon level
/// The list needs to be passed as a list of tuples (min_level:i32, weight:i32)
/// If no eligible weight can be found 0 will be returned
pub fn by_dungeon_level(chance_table: Cow<Vec<(i32, i32)>>, level: u8) -> i32 {
    let mut value_of_level = 0;
    chance_table.iter().for_each(|(value, min_level)|{
        if level as i32 >= *min_level {
            value_of_level = *value;
        }
    });
    value_of_level
}