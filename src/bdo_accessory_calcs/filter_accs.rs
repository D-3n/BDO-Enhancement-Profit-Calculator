
use crate::bdo_market_requests::CategoryGivenInfo;
pub fn filter_accessories_category(accessories: Vec<CategoryGivenInfo>, grade_filter: u8, min_price: u64, max_price: u64) -> Vec<CategoryGivenInfo> {
    let accs = accessories
        .into_iter()
        .filter(|acc| acc.item_grade == grade_filter && acc.base_price >= min_price && acc.base_price <= max_price)
        .collect::<Vec<CategoryGivenInfo>>();
    accs
}
