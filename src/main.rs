use bdo_enhancement_profit_calculator::accessories::{self, get_tap_proft};
use bdo_enhancement_profit_calculator::bdo_market_requests::bdo_post_requests::get_item_buy_sell_info;
use bdo_enhancement_profit_calculator::bdo_market_requests::{
    CategoryGivenInfo, ItemBuySellInfo, ItemID, RegQueueItem,
};
use std::io;

use {
    bdo_enhancement_profit_calculator::bdo_market_requests::bdo_post_requests::get_items_from_category,
    bdo_enhancement_profit_calculator::general_calcs::market_calcs::{
        calc_profit, calc_profit_taxed, get_market_tax,
    },
};

fn get_region() -> String {
    let mut inp_region = String::new();
    println!("Please enter the region for which you wish to the run the commands: ");
    io::stdin().read_line(&mut inp_region).unwrap();

    inp_region.trim().to_lowercase()
}

fn main() {
    let str_inp_region = get_region();
    let str_inp_region = str_inp_region.as_str();

    let rings = get_items_from_category(str_inp_region, 20, 1).unwrap();
    let necklaces = get_items_from_category(str_inp_region, 20, 2).unwrap();
    let earrings = get_items_from_category(str_inp_region, 20, 3).unwrap();
    let belts = get_items_from_category(str_inp_region, 20, 4).unwrap();

    let mut rings = CategoryGivenInfo::build_vec(rings).unwrap();
    let mut necklaces = CategoryGivenInfo::build_vec(necklaces).unwrap();
    let mut earrings = CategoryGivenInfo::build_vec(earrings).unwrap();
    let mut belts = CategoryGivenInfo::build_vec(belts).unwrap();

    let mut accessories = Vec::new();
    accessories.append(&mut rings);
    accessories.append(&mut necklaces);
    accessories.append(&mut earrings);
    accessories.append(&mut belts);

    let accessories = accessories::filter_accessories_category(accessories, 3, 10000000, u64::MAX);

    for acc in accessories {
        let id = acc.get_item_id().to_string();
        let id = &id;
        let base_info = get_item_buy_sell_info(str_inp_region, id, "0").unwrap();
        let tet_info = get_item_buy_sell_info(str_inp_region, id, "4").unwrap();
        let base_info = ItemBuySellInfo::build_vec(base_info).unwrap();
        let tet_info = ItemBuySellInfo::build_vec(tet_info).unwrap();

        let mut sold_price = base_info.get_lowest_listed();
        if sold_price == u64::MAX {
            sold_price = base_info.get_max_price();
        }

        if sold_price * 73 < tet_info.get_base_price() && !acc.get_item_name().contains("Manos") {
            println!("---------------------------------------------------");
            println!("Name: {}", acc.get_item_name());
            println!(
                "Buy at: {} || Sell at : {}",
                sold_price,
                tet_info.get_base_price()
            );
            println!(
                "Profit: {}",
                calc_profit(
                    (sold_price * 73).try_into().unwrap(),
                    tet_info.get_base_price().try_into().unwrap()
                )
            );
            println!(
                "Profit after tax: {}",
                calc_profit_taxed(
                    sold_price * 73,
                    tet_info.get_base_price().try_into().unwrap(),
                    get_market_tax(4500, true, false)
                )
            );
        }
    }
    io::stdin().read_line(&mut String::new()).unwrap();
}
