use super::get_bdo_urls::{create_post_url, get_market_url};
use reqwest;
use serde_json::{json, Value};
use std::env;
/**
 * Copied from https://gist.github.com/guy0090/0a8b7a1e69b03702bb45fb66a05dced5#file-bdowebmarket-js
 *
 * To get your TradeAuth_Session and __RequestVerificationToken cookies,
 * you need to login to your region"s web trade market.
 *
 * Below is a list of all regions and their respective web market URLs, choose
 * your region and login to the web market. Once logged in, open your DevTools
 * (in Chrome: Ctrl + Shift + I or F12) and click the Network tab.
 *
 * On the Web Market, click on a category and then subcategory from the
 * left side menu of the page and check your Network tab.
 *
 * You should see a "GetWorldMarketList" request, click on it
 * and check the "Cookies" tab.
 *
 * Copy the TradeAuth_Session (COOKIE_TRADE_AUTH) and
 * __RequestVerificationToken (COOKIE_REQUEST_VERIFICATION_TOKEN) cookie
 * and set their respective values in the constants below.
 *
 * The TradeAuth_Session cookie doesn"t actually need to be filled in, however
 * it must not be undefined.
 *
 * Now, from the "Payload" tab, copy the __RequestVerificationToken value and set
 * QUERY_REQUEST_VERFICATION_TOKEN to it"s value.
 */

const TRADE_AUTH_SESSION: &str = "TradeAuth_Session";
const REQUEST_VERIFICATION: &str = "__RequestVerificationToken";

// TradeAuth_Session Cookie
const COOKIE_TRADE_AUTH: &'static str = env!("BDO_COOKIE_TRADE_AUTH");

// __RequestVerificationToken Cookie
const COOKIE_REQUEST_VERIFICATION_TOKEN: &'static str =
    env!("BDO_COOKIE_REQUEST_VERIFICATION_TOKEN");

// __RequestVerificationToken URL Encoded Param
const QUERY_REQUEST_VERFICATION_TOKEN: &'static str = env!("BDO_QUERY_REQUEST_VERFICATION_TOKEN");
/*
* These functions parse the results of post requests.
* The data is extracted from the json returned, and made into a string.
*/

// Takes resultMsg (works for trademarket functions)
fn get_result_msg_data(body_text: String) -> Result<String, serde_json::Error> {
    let v: Value = serde_json::from_str(&body_text)?;

    let required_data = v["resultMsg"].to_string();

    let required_data = &required_data[1..required_data.len() - 1];

    Ok(required_data.to_string())
}

// get_items_from_category
fn get_result_msg_market_list(body_text: String) -> Result<String, serde_json::Error> {
    let v: Value = serde_json::from_str(&body_text)?;

    let required_data = v["marketList"].to_string();

    Ok(required_data)
}

/*
* The following functions make post requests to the bdo api
* There are two types, with the second requiring valid cookies (and therefore a BDO account that can access the market.)
*/

pub fn get_item_info(region: &str, item_id: u16) -> Result<String, reqwest::Error> {
    // Type needs to be declared
    let zero: u8 = 0;
    let data = json!({"keyType": zero, "mainKey": item_id});

    // Post request
    let client = reqwest::blocking::Client::new();
    let res = client
        .post(create_post_url(
            get_market_url(region),
            "/Trademarket/GetWorldMarketSubList",
        ))
        .json(&data)
        .send()?;

    // Get String of data seperated by "|"
    let res_body = res.text()?;

    let result_msg = get_result_msg_data(res_body).unwrap();

    Ok(result_msg.to_string())
}

pub fn get_item_price_history(
    region: &str,
    item_id: u16,
    enhancement_id: u8,
) -> Result<String, reqwest::Error> {
    let zero: u8 = 0;
    let data = json!({"keyType": zero, "mainKey": item_id, "subKey": enhancement_id});

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(create_post_url(
            get_market_url(region),
            "/Trademarket/GetMarketPriceInfo",
        ))
        .json(&data)
        .send()?;

    let res_body = res.text()?;
    let result_msg = get_result_msg_data(res_body).unwrap();

    Ok(result_msg.to_string())
}

pub fn get_registration_queue(region: &str) -> Result<String, reqwest::Error> {
    let data = json!({});

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(create_post_url(
            get_market_url(region),
            "/Trademarket/GetWorldMarketWaitList",
        ))
        .json(&data)
        .send()?;

    let res_body = res.text()?;
    let result_msg = get_result_msg_data(res_body).unwrap();

    Ok(result_msg.to_string())
}

pub fn search_market_by_id(region: &str, item_ids: Vec<&str>) -> Result<String, reqwest::Error> {
    let data = json!({"searchResult": item_ids.join(",")});

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(create_post_url(
            get_market_url(region),
            "/Trademarket/GetWorldMarketSearchList",
        ))
        .json(&data)
        .send()?;

    let res_body = res.text()?;
    let result_msg = get_result_msg_data(res_body).unwrap();

    Ok(result_msg.to_string())
}

pub fn get_items_from_category(
    region: &str,
    main_category_no: u16,
    sub_category_no: u16,
) -> Result<String, reqwest::Error> {
    let main_category_no = main_category_no.to_string();
    let sub_category_no = sub_category_no.to_string();

    let params = [
        (REQUEST_VERIFICATION, QUERY_REQUEST_VERFICATION_TOKEN),
        ("mainCategory", &main_category_no),
        ("subCategory", &sub_category_no),
    ];

    let client = reqwest::blocking::Client::new();
    let res = client.post(create_post_url(get_market_url(region), "/Home/GetWorldMarketList"))
        .header("Cookie", format!("{}={}; {}={}", TRADE_AUTH_SESSION, COOKIE_TRADE_AUTH, REQUEST_VERIFICATION, COOKIE_REQUEST_VERIFICATION_TOKEN))
        .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36")
        .form(&params)
        .send()?;

    let res_body = res.text()?;

    let result_msg = get_result_msg_market_list(res_body).unwrap();

    Ok(result_msg)
}

pub fn get_item_buy_sell_info(
    region: &str,
    item_id: &str,
    enhancement_id: &str,
) -> Result<String, reqwest::Error> {
    let params = [
        (REQUEST_VERIFICATION, QUERY_REQUEST_VERFICATION_TOKEN),
        ("keyType", "0"),
        ("mainKey", item_id),
        ("subKey", enhancement_id),
        ("isUp", "true"),
    ];

    let client = reqwest::blocking::Client::new();
    let res = client.post(create_post_url(get_market_url(region), "/Home/GetItemSellBuyInfo"))
        .header("Cookie", format!("{}={}; {}={}", TRADE_AUTH_SESSION, COOKIE_TRADE_AUTH, REQUEST_VERIFICATION, COOKIE_REQUEST_VERIFICATION_TOKEN))
        .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36")
        .form(&params)
        .send()?;

    let res_body = res.text()?;
    Ok(res_body)
}
