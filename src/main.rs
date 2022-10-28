use std::io;
use bdo_enhancement_profit_calculator::accessories;
use bdo_enhancement_profit_calculator::bdo_market_requests::{bdo_post_requests::{get_item_buy_sell_info}, sort_buy_sell_info, sort_category_given_info};

use {bdo_enhancement_profit_calculator::bdo_market_requests::{bdo_post_requests::get_items_from_category}, bdo_enhancement_profit_calculator::general_calcs::market_calcs::{calc_profit, get_market_tax, calc_profit_taxed}};

fn main() {

    let rings = get_items_from_category("eu", 20, 1).unwrap();
    let necklaces = get_items_from_category("eu", 20, 2).unwrap();
    let earrings = get_items_from_category("eu", 20, 3).unwrap();
    let belts = get_items_from_category("eu", 20, 4).unwrap();
    
    let mut rings = sort_category_given_info(rings).unwrap();
    let mut necklaces = sort_category_given_info(necklaces).unwrap();
    let mut earrings = sort_category_given_info(earrings).unwrap();
    let mut belts = sort_category_given_info(belts).unwrap();

    let mut accessories = Vec::new();
    accessories.append(&mut rings);
    accessories.append(&mut necklaces);
    accessories.append(&mut earrings);
    accessories.append(&mut belts);

    let accessories = accessories::filter_accessories_category(accessories, 3, 10000000, u64::MAX);


    for acc in accessories {
        let id = acc.item_id.to_string();
        let id = &id;
        let base_info = get_item_buy_sell_info("eu", id, "0").unwrap();
        let tet_info = get_item_buy_sell_info("eu", id, "4").unwrap();
        let base_info = sort_buy_sell_info(base_info).unwrap();
        let tet_info = sort_buy_sell_info(tet_info).unwrap();

        let mut sold_price = base_info.get_lowest_listed();
        if sold_price == u64::MAX {sold_price = base_info.get_max_price();}

        if sold_price * 73 < tet_info.base_price && !acc.item_name.contains("Manos") {
            println!("---------------------------------------------------");
            println!("Name: {}", acc.item_name);
            println!("Buy at: {} || Sell at : {}", sold_price, tet_info.base_price);
            println!("Profit: {}", calc_profit((sold_price * 73).try_into().unwrap(), tet_info.base_price.try_into().unwrap()));
            println!("Profit after tax: {}", calc_profit_taxed(sold_price * 73, tet_info.base_price.try_into().unwrap(), get_market_tax(4500, true, false)));
        }   
    }
    io::stdin().read_line(&mut String::new()).unwrap();
}

