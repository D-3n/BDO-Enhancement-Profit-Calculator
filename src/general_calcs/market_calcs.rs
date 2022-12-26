//! # Market Calculations
//!
//! These calculations work out profit from selling on the market.

/// Gets the amount that should be taxed.
///
/// # Example
///
/// ```
/// use bdo_enhancement_profit_calculator::general_calcs::market_calcs::get_market_tax;
///
/// let vp = true;
/// let merch = false;
/// let fame: u16 = 5000;
///
/// let tax_rate = get_market_tax(fame, vp, merch);
///
/// assert_eq!(0.8515, tax_rate);
/// ```
///
pub fn get_market_tax(fam_fame: u16, value_pack: bool, merch_ring: bool) -> f64 {
    let base_silver = 0.65;
    let mut bonus: f64 = 0.0;

    if value_pack {
        bonus += 0.3
    };
    if merch_ring {
        bonus += 0.05
    };

    if fam_fame >= 7000 {
        bonus += 0.015;
    } else if fam_fame >= 4000 {
        bonus += 0.01;
    } else if fam_fame >= 1000 {
        bonus += 0.005;
    };
    return base_silver + (base_silver * bonus);
}

/// Calculates the profit made from an item.
///
/// # Example
///
/// ```
/// use bdo_enhancement_profit_calculator::general_calcs::market_calcs::calc_profit;
///
/// let profit = calc_profit(1000, 950);
///
/// assert_eq!(-50, profit);
/// ```
///
pub fn calc_profit(cost_to_make: u64, item_value: u64) -> i64 {
    let cost_to_make = cost_to_make as i64;
    let item_value = item_value as i64;
    return item_value - cost_to_make;
}

/// Calculates the profit made from an item after tax.
///
/// # Example
///
/// ```
/// use bdo_enhancement_profit_calculator::general_calcs::market_calcs::calc_profit_taxed;
///
/// let tax_rate = 0.8;
/// let profit = calc_profit_taxed(1000, 2000, tax_rate);
///
/// assert_eq!(600, profit);
/// ```
///
pub fn calc_profit_taxed(cost_to_make: u64, item_value: u64, tax_rate: f64) -> i64 {
    let taxed_item_value = item_value as f64 * tax_rate;
    let profit = taxed_item_value - cost_to_make as f64;

    return profit.floor() as i64;
}
