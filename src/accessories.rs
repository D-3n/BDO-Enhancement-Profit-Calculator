//! # Accessories
//!
//! Accessories is a collection of commands related to accessories, and how their enhancement works.

use crate::bdo_market_requests::{
    bdo_post_requests::get_item_buy_sell_info, CategoryGivenInfo, HasId, ItemBuySellInfo,
};

/// Filters accessories by grade, a minimum price, and a maximum price.
///
pub fn filter_accessories_category(
    accessories: Vec<CategoryGivenInfo>,
    grade_filter: u8,
    min_price: u64,
    max_price: u64,
) -> Vec<CategoryGivenInfo> {
    let accs = accessories
        .into_iter()
        .filter(|acc| {
            acc.get_item_grade() == grade_filter
                && acc.get_base_price() >= min_price
                && acc.get_base_price() <= max_price
        })
        .collect::<Vec<CategoryGivenInfo>>();
    accs
}

/// Calculates the success chance of enhancing an accessory.
///
/// # Panics
///
/// If the enhancement level is not a valid enhancement (1 - 5 for PRI - PEN)
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
    if enhancement_level < 1 || enhancement_level > 5 {
        panic!("Enhancement level of {enhancement_level}, was given, when it should be in the range 1-5.");
    }

    let chance: f64;
    let failstack = failstack as f64;

    match enhancement_level {
        1 => {
            if failstack > 18.0 {
                chance = 0.25 + (18.0 * 0.025) + ((failstack - 18.0) * 0.005)
            } else {
                chance = 0.25 + (failstack * 0.025)
            }
        }
        2 => {
            if failstack > 40.0 {
                chance = 0.1 + (40.0 * 0.01) + ((failstack - 40.0) * 0.002)
            } else {
                chance = 0.1 + (failstack * 0.01)
            }
        }
        3 => {
            if failstack > 44.0 {
                chance = 0.075 + (44.0 * 0.0075) + ((failstack - 44.0) * 0.0015)
            } else {
                chance = 0.075 + (failstack * 0.0075)
            }
        }
        4 => {
            if failstack > 110.0 {
                chance = 0.025 + (110.0 * 0.0025) + ((failstack - 110.0) * 0.0005)
            } else {
                chance = 0.025 + (failstack * 0.0025)
            }
        }
        5 => {
            if failstack > 390.0 {
                chance = 0.005 + (390.0 * 0.0005) + ((failstack - 390.0) * 0.0001)
            } else {
                chance = 0.005 + (failstack * 0.0005)
            }
        }
        _ => chance = 0.0,
    }

    if chance > 0.9 {
        return 0.9;
    };

    chance
}

/// Calculates accessories required to enhance from base to a certain level.
///
/// # Panics
///
/// If there are less stacks then required
/// If the enhancement level is too high/low
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
/// let amount = bdo_enhancement_profit_calculator::accessories::accessories_required(1, stacks);
///
/// assert_eq!(3, amount);
/// ```
///
pub fn accessories_required(end_enhancement: u8, stacks: Vec<u16>) -> u16 {
    if end_enhancement < 1 || end_enhancement > 5 {
        panic!("Enhancement level of {end_enhancement}, was given, when it should be in the range 1-5.");
    };
    if stacks.len() < (end_enhancement as usize) {
        panic!("Not enough stacks were supplied.");
    };

    let mut amount: f64 = 1.0;
    let mut i: u8 = 1;
    for stack in stacks {
        amount = (1.0 / calc_accessory_chance(i, stack)) * (amount + 1.0);
        if i >= end_enhancement {
            break;
        }
        i += 1;
    }

    amount.ceil() as u16
}

/// Calculates accessories required to enhance from base to a certain level, and does not round.
///
/// # Panics
///
/// If there are less stacks then required
/// If the enhancement level is too high/low
///
/// # Examples
///
/// ```
/// let stacks = vec![20, 40, 44, 110, 250];
/// let amount = bdo_enhancement_profit_calculator::accessories::accessories_required_exact(4, stacks);
///
/// assert_eq!(74.39343882223382, amount);
/// ```
///
/// ```
/// let stacks = vec![20];
/// let amount = bdo_enhancement_profit_calculator::accessories::accessories_required_exact(1, stacks);
///
/// assert_eq!(2.8169014084507045, amount);
/// ```
///
pub fn accessories_required_exact(end_enhancement: u8, stacks: Vec<u16>) -> f64 {
    if end_enhancement < 1 || end_enhancement > 5 {
        panic!("Enhancement level of {end_enhancement}, was given, when it should be in the range 1-5.");
    };
    if stacks.len() < (end_enhancement as usize) {
        panic!("Not enough stacks were supplied.");
    };

    let mut amount: f64 = 1.0;
    let mut i: u8 = 1;
    for stack in stacks {
        amount = (1.0 / calc_accessory_chance(i, stack)) * (amount + 1.0);
        if i >= end_enhancement {
            break;
        }
        i += 1;
    }

    amount
}

/// Calculates the average profit from a singular tap.
///
/// # Panics
///
/// If the given item has an enhancement level above TET (4)
///
pub fn get_tap_proft<T: HasId>(item_id: T, level: u8, stack: u16, region: &str) -> i64 {
    let id = item_id.get_item_id();
    if level > 4 {
        panic!("An enhancement level greater than 4 (TET, max - 1 for accessories) was supplied.")
    };

    let item = get_item_buy_sell_info(region, &id.to_string(), &level.to_string()).unwrap();
    let item = ItemBuySellInfo::build_vec(item).unwrap();
    let upgrade_item =
        get_item_buy_sell_info(region, &id.to_string(), &(level + 1).to_string()).unwrap();
    let upgrade_item = ItemBuySellInfo::build_vec(upgrade_item).unwrap();

    let base_item = get_item_buy_sell_info(region, &id.to_string(), "0").unwrap();
    let base_item = ItemBuySellInfo::build_vec(base_item).unwrap();
    let chance = calc_accessory_chance(level + 1, stack);

    let make_cost =
        (base_item.get_lowest_listed() + item.get_lowest_listed()) as f64 * (1.0 / chance);

    upgrade_item.get_base_price() as i64 - make_cost.ceil() as i64
}
