/// Gets the url of a region's market given the abbreviation of the region.
///
/// # Panics
/// 
/// If the given region is not valid.
/// 
/// # Examples
/// 
/// ```
/// use bdo_enhancement_profit_calculator::bdo_market_requests::get_bdo_urls::get_market_url;
/// 
/// let link = get_market_url("na");
/// assert_eq!("https://na-trade.naeu.playblackdesert.com", link);
/// ```
/// 
pub fn get_market_url(region: &str) -> &str {

    match region {
        "na" => "https://na-trade.naeu.playblackdesert.com",
        "eu" => "https://eu-trade.naeu.playblackdesert.com",
        "sea" => "https://trade.sea.playblackdesert.com",
        "mena" => "https://trade.tr.playblackdesert.com",
        "kr" => "https://trade.kr.playblackdesert.com",
        "ru" => "https://trade.ru.playblackdesert.com",
        "jp" => "https://trade.jp.playblackdesert.com",
        "th" => "https://trade.th.playblackdesert.com",
        "tw" => "https://trade.tw.playblackdesert.com",
        "sa" => "https://trade.sa.playblackdesert.com",
        "console_eu" => "https://eu-trade.console.playblackdesert.com",
        "console_na" => "https://na-trade.console.playblackdesert.com",
        "console_asia" => "https://asia-trade.console.playblackdesert.com",
        oth => {
            println!("got {}", oth);
            panic!("Invalid region entered!")
            },
    }
}

/// Appends a string to the given url.
///
/// # Examples
/// 
/// ```
/// use bdo_enhancement_profit_calculator::bdo_market_requests::get_bdo_urls::create_post_url;
/// 
/// let link = create_post_url("https://www.example.com", "/test");
/// 
/// assert_eq!("https://www.example.com/test", link);
/// ```
/// 
pub fn create_post_url(region_url: &str, post_req_subdirectory: &str) -> String {
    let mut url = region_url.to_owned();
    url.push_str(post_req_subdirectory);
    url
}
