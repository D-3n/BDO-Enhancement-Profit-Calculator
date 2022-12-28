//! # Accessories
//!
//! Accessories is a collection related to accessories, and how their enhancement works.


use crate::bdo_market_requests::{CategoryGivenInfo, HasId, ItemBuySellInfo, ItemID};
/// Contains data about how an accessory will be enhanced.
/// 
pub struct AccEnhancementDetails<T: HasId> {
    item_id: T,
    level: u8,
    stacks: Option<Vec<u16>>
}

impl<T: HasId> HasId for AccEnhancementDetails<T> {
    fn get_item_id(&self) -> u32 {
        self.item_id.get_item_id()
    }
    
}

impl<T: HasId> AccEnhancementDetails<T> {
    pub fn new(item: T, level: u8, stacks: Option<Vec<u16>>) -> Self {
        Self {
            item_id: item,
            level: level,
            stacks: stacks
        }
    }

    pub fn get_stacks(&self) -> Option<&Vec<u16>> {
        self.stacks.as_ref()
    }

    pub fn get_level(&self) -> u8 {
        self.level
    }

    /// Gets the first stack
    /// 
    /// # Panics
    /// 
    /// If there are no stacks.
    /// 
    fn get_first_stack(&self) -> u16 {
        match &self.stacks {
            None => panic!("There are no stacks when at least 1 is required."),
            Some(v) => {
                if let Some(_) = v.get(0) {
                    return  v[0];
                } else {
                    panic!("There are no stacks when at least 1 is required.")
                }
            }
        }
    }
}

/// Contains data of what makes an enhancement profitable.
///
pub struct AccProfitDetails {
    make_cost: u64,
    actual_value: u64,
    profit: i64,
    profit_taxed: i64
} 
impl AccProfitDetails {
    pub fn new(make_cost: u64, actual_value: u64, tax_rate: f64) -> Self {
        let profit = actual_value as i64 - make_cost as i64;
        let actual_value_taxed = actual_value as f64 * tax_rate;
        let profit_taxed = actual_value_taxed.ceil() as i64 - make_cost as i64;

        AccProfitDetails { make_cost, actual_value, profit, profit_taxed }
    }

    pub fn get_make_cost(&self) -> u64 {
        self.make_cost
    }

    pub fn get_actual_value(&self) -> u64 {
        self.actual_value
    }

    pub fn get_profit(&self) -> i64 {
        self.profit
    }

    pub fn get_profit_taxed(&self) -> i64 {
        self.profit_taxed
    }
}
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
/// Calculates the cost of a singular tap.
/// 
/// # Panics
/// 
/// If the given item has an enhancement level above TET (4)
/// 
pub fn get_tap_cost<T: HasId>(item_details: AccEnhancementDetails<T>, item_cost: Option<u64>, region: &str) -> u64 {

    
    let id = item_details.get_item_id();
    let level = item_details.get_level();
    let stack = item_details.get_first_stack();

    let cost = match item_cost {
        Some(v) => v,
        None => {
            let item = ItemBuySellInfo::from_post(region, &id.to_string(), &level.to_string());
            match item {
                Ok(v) => v.get_lowest_listed(),
                Err(_) => 0
            }
        }
    };


    if level > 4 {
        panic!("An enhancement level greater than 4 (TET, max - 1 for accessories) was supplied.")
    };

    
    let base_item = ItemBuySellInfo::from_post(region, &id.to_string(), "0").unwrap(); // Level of 0 should always be safe.
    let chance = calc_accessory_chance(level + 1, stack);

    let make_cost =
        (base_item.get_lowest_listed() + cost) as f64 * (1.0 / chance);

    make_cost.ceil() as u64
}
/// Calculates the average profit from a singular tap.
///
/// # Panics
///
/// If the given item has an enhancement level above TET (4)
///
pub fn get_tap_proft<T: HasId>(item_details: AccEnhancementDetails<T>, item_cost: Option<u64>, tax_rate: f64, region: &str) -> AccProfitDetails {

    let id = item_details.get_item_id();
    let level = item_details.get_level();

    if level > 4 {
        panic!("An enhancement level greater than TET (4) was given.")
    }
    let make_cost = get_tap_cost(item_details, item_cost, region);
    // println!("calling on region: {}, id: {}, level: {}", region, &id.to_string(), &(level+1).to_string());

    let upgrade_item =
        ItemBuySellInfo::from_post(region, &id.to_string(), &(level + 1).to_string());

    match upgrade_item {
        Ok(v) => AccProfitDetails::new(make_cost, v.get_base_price(), tax_rate),
        Err(_) => AccProfitDetails::new(1,0,0.0)
    }

    
}
/// Calculates profit to tap from [Enhancement 1] to [Enhancement 2]
/// 
/// # Panics 
/// 
/// If enhancement 2 >= Enhancement 1
/// If either is >5 or <0
/// If not enough stacks are provided.
/// 
pub fn get_tap_profit_mult<T: HasId>(item_details: AccEnhancementDetails<T>, end_level: u8, tax_rate: f64, region: &str) -> AccProfitDetails {
    let id = item_details.get_item_id();
    let current_level = item_details.get_level();
    let stacks = item_details.get_stacks();

    let stacks = if let Some(v) = stacks {
        v
    } else {
        panic!("No stacks were provided, when one or more were required.")
    };

    if current_level > 4 {
        panic!("The starting level was too high for enhancement to be possible.")
    } 

    if end_level <= current_level || end_level > 5 {
        panic!("It is impossible to enhance {current_level} to {end_level}.")
    }

    // Declared here to prevent underflow
    let level_gap = end_level - current_level;

    if stacks.len() < (level_gap).into() {
        panic!("Not enough stacks were provided (needed {level_gap}).")
    }

    let mut cost: u64 = 0;

    // Loops 1 time less than necessary, so the final call can be get_tap_profit
    // This allows for easy access to the final actual value, removing the need for redundant API calls.
    // Looping 0..0 will entirely skip the for loop, and is therefore valid.
    for i in 0..(level_gap - 1).into() {
        let details = AccEnhancementDetails::new(ItemID::new(id), current_level + i, Some(vec![stacks[i as usize]])); // Takes only the current stack
        let single_cost = if i == 0 {
            get_tap_cost(details, None, region)
        } else { 
            get_tap_cost(details, Some(cost), region)
        };
        cost += single_cost;
    }

    let details = AccEnhancementDetails::new(ItemID::new(id), end_level - 1, Some(vec![*stacks.last().unwrap()])); // Impossible that it's empty, safe to unwrap.
    let last_cost = get_tap_proft(details, Some(cost), tax_rate, region);

    let ac_val = last_cost.get_actual_value();
    let total_cost = cost + last_cost.get_make_cost();

    let profit_data = AccProfitDetails::new(total_cost, ac_val, tax_rate);
    profit_data

}