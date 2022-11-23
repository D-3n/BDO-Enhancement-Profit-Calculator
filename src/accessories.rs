//! # Accessories
//! 
//! Accessories is a collection of commands related to accessories, and how their enhancement works.

use crate::bdo_market_requests::CategoryGivenInfo;

/// Filters accessories by grade, a minimum price, and a maximum price.
///
pub fn filter_accessories_category(accessories: Vec<CategoryGivenInfo>, grade_filter: u8, min_price: u64, max_price: u64) -> Vec<CategoryGivenInfo> {
    let accs = accessories
        .into_iter()
        .filter(|acc| acc.item_grade == grade_filter && acc.base_price >= min_price && acc.base_price <= max_price)
        .collect::<Vec<CategoryGivenInfo>>();
    accs
}

/// Calculates the success chance of enhancing an accessory.
/// 
/// # Panics
/// 
/// If the given enhancement level is greater than 5.
/// 
/// 
/// # Examples
/// 
/// ```
/// let stack = 17;
/// let chance = bdo_enhancement_profit_calculator::accessories::calc_accessory_chance(1, stack);
/// 
/// assert_eq!(0.675, chance);
/// ```
/// 
/// ```
/// let stack = 110;
/// let chance = bdo_enhancement_profit_calculator::accessories::calc_accessory_chance(5, stack);
/// 
/// assert_eq!(0.06, chance);
/// ```
/// 
pub fn calc_accessory_chance(enhancement_level: u8, failstack: u16) -> f64 {
    
    if enhancement_level > 5 || enhancement_level < 1 {
        panic!("The range for accessories is 0-5 (BASE - PEN), level given was {enhancement_level}");
    }

    let chance: f64;
    let failstack: f64 = failstack as f64;

    match enhancement_level {
        1 => if failstack > 18.0 {chance = 0.25 + (18.0 * 0.025) + ((failstack - 18.0) * 0.005)} else {chance = 0.25 + (failstack * 0.025)}
        2 => if failstack > 40.0 {chance = 0.1 + (40.0 * 0.01) + ((failstack - 40.0) * 0.002)} else {chance = 0.1 + (failstack * 0.01)}
        3 => if failstack > 44.0 {chance = 0.075 + (44.0 * 0.0075) + ((failstack - 44.0) * 0.0015)} else {chance = 0.075 + (failstack * 0.0075)}
        4 => if failstack > 110.0 {chance = 0.025 + (110.0 * 0.0025) + ((failstack - 110.0) * 0.0005)} else {chance = 0.025 + (failstack * 0.0025)}
        5 => if failstack > 390.0 {chance = 0.005 + (390.0 * 0.0005) + ((failstack - 390.0) * 0.0001)} else {chance = 0.005 + (failstack * 0.0005)}
        _ => panic!("Impossible result for enhancement level: {enhancement_level}")
    }

    if chance > 0.9 {return 0.9}; // No matter the stack, enhancements can't have more than a 90% chance.

    chance
}

/// Calculates accessories required to enhance from base to a certain level.
/// 
/// # Panics
/// 
/// If the given enhancement level is greater than 5.
/// 
/// 
/// # Examples
/// 
/// ```
/// let stacks = vec![20, 40, 44, 110, 250];
/// let amount = bdo_enhancement_profit_calculator::accessories::accessories_required(4, stacks);
/// 
/// assert_eq!(75, amount);
/// ```
/// 
/// ``` 
/// let stacks = vec![20];
/// let amount = bdo_enhancement_profit_calculator::accessories::accessories_required(2, stacks);
/// 
/// assert_eq!(3, amount);
/// ```
/// 
pub fn accessories_required(end_enhancement: u8, stacks: Vec<u16>) -> u16 {

    if end_enhancement < 1 || end_enhancement > 5 {
        panic!("The range for accessories is 0-5 (BASE - PEN), level given was {end_enhancement}");
    }
    let mut amount: f64 = 1.0;
    let mut i: u8 = 1;
    for stack in stacks {
        amount = (1.0 / calc_accessory_chance(i, stack)) * (amount + 1.0);
        if i >= end_enhancement {break}
        i += 1;
    }

    amount.ceil() as u16
}
