// calc_profit, get_market_tax, calc_profit_taxed

pub fn get_market_tax(fam_fame: u16, value_pack: bool, merch_ring: bool) -> f64 {
    let base_silver = 0.65;
    let mut bonus: f64 = 0.0;

    if value_pack {bonus += 0.3};
    if merch_ring {bonus += 0.05};

    if fam_fame >= 1000 {
        bonus += 0.005;
    } else if fam_fame >= 4000 {
        bonus += 0.01;
    } else if fam_fame >= 7000 {
        bonus += 0.015;
    };

    return base_silver + (base_silver * bonus)
}

pub fn calc_profit(cost_to_make : u64, item_value: u64) -> i64 {
    let cost_to_make = cost_to_make as i64;
    let item_value = item_value as i64;
    return item_value - cost_to_make
}


pub fn calc_profit_taxed(cost_to_make : u64, item_value: u64, tax_rate: f64) -> i64 {
    let taxed_item_value = item_value as f64 * tax_rate;
    let profit = taxed_item_value - cost_to_make as f64;

    return profit.floor() as i64
}


/*
function getMarketTax(familyFame = 1500, valuePack = true, merchantRing = false) {
    const SILVEREARNED = 0.65
    let bonus = 0

    if (valuePack) {bonus += 0.3}
    if (merchantRing) {bonus += 0.05}

    if (familyFame >= 1000) {bonus += 0.005}
        else if (familyFame >= 4000) {bonus += 0.01}
        else if (familyFame >= 7000) {bonus += 0.015}

    return (SILVEREARNED + (SILVEREARNED * bonus)) // Total silver earned

}
 */