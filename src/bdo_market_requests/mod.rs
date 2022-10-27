pub mod bdo_post_requests;
pub mod get_bdo_urls;

use serde_json::{Value, Error};

// Get item info
pub struct SpecificItemInfo {
    item_id: u32,
    enhancement_min: u8,
    enhancement_max: u8,
    base_price: u64,
    stock: u64,
    total_trades: u64,
    price_cap_min: u64,
    price_cap_max: u64,
    last_sale_price: u64,
    last_sale_time: String // Unix timestamp
}

pub fn sort_specific_item_info(data: String) -> Vec<SpecificItemInfo> {
    let outer_split = data.split("|").filter(|f| f != &"");

    let mut item_info: Vec<SpecificItemInfo> = Vec::new();
    for s in outer_split {

        let inner_split = s.split("-");
        
        let vec: Vec<&str> = inner_split.collect();
        let single_item_info = SpecificItemInfo {
            item_id: vec[0].parse::<u32>().unwrap_or(0),
            enhancement_min: vec[1].parse::<u8>().unwrap_or(0),
            enhancement_max: vec[2].parse::<u8>().unwrap_or(0),
            base_price: vec[3].parse::<u64>().unwrap_or(0),
            stock: vec[4].parse::<u64>().unwrap_or(0),
            total_trades: vec[5].parse::<u64>().unwrap_or(0),
            price_cap_min: vec[6].parse::<u64>().unwrap_or(0),
            price_cap_max: vec[7].parse::<u64>().unwrap_or(0),
            last_sale_price: vec[8].parse::<u64>().unwrap_or(0),
            last_sale_time: vec[9].to_string(),
        };

        item_info.push(single_item_info)
    }

    item_info
}

// Get item price history doesn't need a struct - list of strings

// Get registration queue
pub struct RegQueueItem {
    item_id: u32,
    enhancement_level: u8,
    listed_price: u64,
    registered_timestamp: String
}


pub fn sort_reg_queue_item_info(data: String) -> Vec<RegQueueItem> {
    let outer_split = data.split("|").filter(|f| f != &"");

    let mut item_info: Vec<RegQueueItem> = Vec::new();
    for s in outer_split {

        let inner_split = s.split("-");
        let vec: Vec<&str> = inner_split.collect();
        let single_item_info = RegQueueItem {
            item_id: vec[0].parse::<u32>().unwrap_or(0),
            enhancement_level: vec[1].parse::<u8>().unwrap_or(0),
            listed_price: vec[2].parse::<u64>().unwrap_or(0),
            registered_timestamp: vec[3].to_string(),
        };

        item_info.push(single_item_info)
    }

    item_info
}


// Search market by id
pub struct SearchedItem {
    item_id: u32,
    stock: u64,
    base_price: u64,
    total_trades: u64
}


pub fn sort_searched_item_info(data: String) -> Vec<SearchedItem> {
    let outer_split = data.split("|").filter(|f| f != &"");

    let mut item_info: Vec<SearchedItem> = Vec::new();
    for s in outer_split {

        let inner_split = s.split("-");
        let vec: Vec<&str> = inner_split.collect();
        let single_item_info = SearchedItem {
            item_id: vec[0].parse::<u32>().unwrap_or(0),
            stock: vec[1].parse::<u64>().unwrap_or(0),
            base_price: vec[2].parse::<u64>().unwrap_or(0),
            total_trades: vec[3].parse::<u64>().unwrap_or(0),
        };

        item_info.push(single_item_info)
    }

    item_info
}

// EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
// IDK

// Code to remove \"\" from string:

fn remove_second_quotes(st: String) -> String {
    let mut st_char = st.chars();
    st_char.next();
    st_char.next_back();
    let st2 = st_char.collect::<String>();
    st2
}

// Get items from category
#[derive(Debug)]
pub struct CategoryGivenInfo {
    pub item_grade: u8,
    pub item_id: u32,
    pub base_price: u64,
    pub item_name: String,
    stock: u64
}

pub fn sort_category_given_info(data: String) -> Result<Vec<CategoryGivenInfo>,Error> {
    let v: Value = serde_json::from_str(&data)?;

    let v = if let serde_json::Value::Array(entries) = v {
        entries
    } else {
        panic!("Data inputted was not an array")
    };
    // v is a Vector of items, stored as type: serde_json::Value (Object)

    let mut item_info: Vec<CategoryGivenInfo> = Vec::new();

    for item in &v {


        let grade: u8 = item["grade"].to_string().parse::<u8>().unwrap();
        let id: u32 = item["mainKey"].to_string().parse::<u32>().unwrap();
        let base_price: u64 = item["minPrice"].to_string().parse::<u64>().unwrap();
        let name: String = remove_second_quotes(item["name"].to_string());
        let stock: u64 = item["sumCount"].to_string().parse::<u64>().unwrap();

        let single_item_info: CategoryGivenInfo = CategoryGivenInfo { 
            item_grade: grade, 
            item_id: id, 
            base_price: base_price, 
            item_name: name, 
            stock: stock 
        };

        item_info.push(single_item_info);

    }

    Ok(item_info)
}


#[derive(Debug)]
// Get item buy/sell info
pub struct BiddingInfo{
    sell_count: u32,
    buy_count: u32,
    bidding_price: u64
}

#[derive(Debug)]
pub struct ItemBuySellInfo { // Impls need testing
    bids: Vec<BiddingInfo>,
    pub base_price: u64,
    enhancement_group: u8,
    enhancement_material_id: u32,
    enhancement_material_base_price: u64,
    enhancement_material_required_amount: u8,
    max_bids_per_person: u16,
}
impl ItemBuySellInfo {
    pub fn get_max_price(&self) -> u64 {
        let mut max: u64 = 0;
        for bid in &self.bids {
            if bid.bidding_price > max {
                max = bid.bidding_price
            }
        }

        max
    }
    pub fn get_min_price(&self) -> u64 {

        let mut min: u64 = u64::MAX;
        for bid in &self.bids {
            if bid.bidding_price < min {
                min = bid.bidding_price
            }
        }

        min
    }
    pub fn get_lowest_listed(&self) -> u64 {
        let mut price: u64 = u64::MAX;
        for bid in &self.bids {
            if bid.sell_count > 0 {
                if bid.bidding_price < price {
                    price = bid.bidding_price;
                }
            }
        }
        price
    }
}

/*
priceList
marketConditionList
basePrice
enchantGroup
enchantMaterialKey
enchantMaterialPrice
enchantNeedCount
maxRegisterForWorldMarket
*/

pub fn sort_buy_sell_info(data: String) -> Result<ItemBuySellInfo, Error> {
    let v: Value = serde_json::from_str(&data)?;

    let max_bids_per_person = v["maxRegisterForWorldMarket"].to_string().parse::<u16>().unwrap();
    let base_price = v["basePrice"].to_string().parse::<u64>().unwrap();
    let enhancement_group = v["enchantGroup"].to_string().parse::<u8>().unwrap();
    let enhancement_material_id = v["enchantMaterialKey"].to_string().parse::<u32>().unwrap();
    let enhancement_material_base_price = v["enchantMaterialPrice"].to_string().parse::<u64>().unwrap();
    let enhancement_material_required_amount = v["enchantNeedCount"].to_string().parse::<u8>().unwrap();

    let all_bids = v["marketConditionList"].clone();

    let all_bids = if let serde_json::Value::Array(entries) = all_bids {
        entries
    } else {
        panic!("Data inputted was not an array")
    };


    // Get vector of bids
    let mut bids_vec: Vec<BiddingInfo> = Vec::new();

    for listing in all_bids {

        let buy: u32 = listing["buyCount"].to_string().parse::<u32>().unwrap();
        let sell: u32 = listing["sellCount"].to_string().parse::<u32>().unwrap();
        let price: u64 = listing["pricePerOne"].to_string().parse::<u64>().unwrap();

        let single_bidding: BiddingInfo = BiddingInfo {
            buy_count: buy,
            sell_count: sell,
            bidding_price: price,
        };
        bids_vec.push(single_bidding)
    }

    let info = ItemBuySellInfo {
        bids: bids_vec,
        base_price: base_price,
        enhancement_group: enhancement_group,
        enhancement_material_id: enhancement_material_id,
        enhancement_material_base_price: enhancement_material_base_price,
        enhancement_material_required_amount: enhancement_material_required_amount,
        max_bids_per_person: max_bids_per_person,
    };
    Ok(info)
}

