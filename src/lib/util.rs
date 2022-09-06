extern crate reqwest;

use reqwest::Error;

use super::config;

pub async fn get_res_robot_response() -> Result<String, Error> {
    // 740000005 - UPPSALA
    // 740098136 - STORVRETA
    let url = format!(
        "https://api.resrobot.se/v2.1/departureBoard?id={}&format=json&accessId={}",
        740098136,
        config::CONFIG.keys.res_robot
    );

    let res = reqwest::get(url).await?.text().await?;

    Ok(res)
}
