//! # Enhancement Calculations
//!
//! These calculations are related to the enhancement of any item.

use rand::Rng;

/// Calculates the total chance of success given multiple attempts.
///
/// # Panics
///
/// If chance > 1
///
/// # Example
///
/// ```
/// use bdo_enhancement_profit_calculator::general_calcs::enhancement_calcs::calc_total_chance;
///
/// let chance = 0.3;
/// let total_chance = calc_total_chance(chance, 2);
///
/// assert_eq!(0.51, total_chance);
/// ```
///
pub fn calc_total_chance(chance: f64, attempt_amount: i32) -> f64 {
    if chance > 1.0 {
        panic!("Chance is greater than 100%! Input chances as decimals.")
    }

    1.0 - ((1.0 - chance).powi(attempt_amount))
}

/// Simulates an enhancement of given chance.
///
/// # Panics
///
/// If chance > 1
///
/// # Example
///
/// ```
/// use bdo_enhancement_profit_calculator::general_calcs::enhancement_calcs::simulate_enhancement;
///
/// let num = simulate_enhancement(0.5);
///
/// if num {
///     println!("You got heads!");
/// } else {
///     println!("You got tails!");
/// }
/// ```
///
pub fn simulate_enhancement(chance: f64) -> bool {
    if chance > 1.0 {
        panic!("Chance is greater than 100%! Input chances as decimals.")
    }

    let simulated_chance: f64 = rand::thread_rng().gen();
    if simulated_chance < chance {
        return true;
    } else {
        return false;
    }
}
