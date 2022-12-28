use bdo_enhancement_profit_calculator::accessories::{self, AccEnhancementDetails, get_tap_profit_mult};

use bdo_enhancement_profit_calculator::bdo_market_requests::CategoryGivenInfo;

use std::io;

fn get_region() -> String {
    let mut inp_region = String::new();
    println!("Please enter the region for which you wish to the run the commands: ");
    io::stdin().read_line(&mut inp_region).unwrap();

    inp_region.trim().to_lowercase()
}

fn main() {

    let str_inp_region = get_region();
    let str_inp_region = str_inp_region.as_str();

    let mut rings = CategoryGivenInfo::from_post(str_inp_region, 20, 1);
    let mut necklaces = CategoryGivenInfo::from_post(str_inp_region, 20, 2);
    let mut earrings = CategoryGivenInfo::from_post(str_inp_region, 20, 3);
    let mut belts = CategoryGivenInfo::from_post(str_inp_region, 20, 4);

    let mut accessories = Vec::new();
    accessories.append(&mut rings);
    accessories.append(&mut necklaces);
    accessories.append(&mut earrings);
    accessories.append(&mut belts);

    let accessories = accessories::filter_accessories_category(accessories, 3, 1000000, u64::MAX);
    let accessories: Vec<CategoryGivenInfo> = accessories.into_iter().filter(|acc| !acc.get_item_name().contains("Manos")).collect();
    for acc in accessories {

        let name = acc.get_item_name().to_owned();
        println!("Checking {}", name);
        let base_price = acc.get_base_price();

        let details = AccEnhancementDetails::new(acc, 0, Some(vec![20, 40, 44, 110]));
        let profit_details = get_tap_profit_mult(details, 4, 0.8515, str_inp_region);
        let p = profit_details.get_profit();
        if p != -1 && p > 50000000 {
            println!("---------------------------------------------------");
            println!("Name: {}", name);
            println!(
                "Buy at: {} || Sell at : {}",
                base_price,
                profit_details.get_actual_value()
            );
            println!(
                "Profit: {}",
                profit_details.get_profit()
            );
            println!(
                "Profit after tax: {}",
                profit_details.get_profit_taxed()
            );
            println!("---------------------------------------------------");
        }
    }
    println!("Done");
    io::stdin().read_line(&mut String::new()).unwrap();
}
