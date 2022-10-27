use rand::Rng;

pub fn calc_accessory_chance(enhancement_level: u8, failstack: u16) -> f64 {
    
    let chance: f64;
    let failstack = failstack as f64;

    match enhancement_level {
        1 => if failstack > 18.0 {chance = 0.25 + (18.0 * 0.025) + ((failstack - 18.0) * 0.005)} else {chance = 0.25 + (failstack * 0.025)}
        2 => if failstack > 40.0 {chance = 0.1 + (40.0 * 0.01) + ((failstack - 40.0) * 0.002)} else {chance = 0.1 + (failstack * 0.01)}
        3 => if failstack > 44.0 {chance = 0.075 + (44.0 * 0.0075) + ((failstack - 44.0) * 0.0015)} else {chance = 0.075 + (failstack * 0.0075)}
        4 => if failstack > 110.0 {chance = 0.025 + (110.0 * 0.0025) + ((failstack - 110.0) * 0.0005)} else {chance = 0.025 + (failstack * 0.0025)}
        5 => if failstack > 390.0 {chance = 0.005 + (390.0 * 0.0005) + ((failstack - 390.0) * 0.0001)} else {chance = 0.005 + (failstack * 0.0005)}
        _ => chance = 0.0
    }

    if chance > 0.9 {return 0.9};

    chance
}

pub fn calc_total_chance(chance: f64, attempt_amount: i32) -> f64 {
    1.0 - ((1.0 - chance).powi(attempt_amount))
}

pub fn simulate_enhancement(chance: f64) -> bool {
    let simulated_chance: f64 = rand::thread_rng().gen();
    if simulated_chance < chance {return true} else {return false}
}

pub fn accessories_required(end_enhancement: u8, stacks: Vec<u16>) -> u16 {
    let mut amount: f64 = 1.0;
    let mut i: u8 = 1;
    for stack in stacks {
        amount = (1.0 / calc_accessory_chance(i, stack)) * (amount + 1.0);
        if i >= end_enhancement {break}
        i += 1;
    }

    amount.ceil() as u16 
}