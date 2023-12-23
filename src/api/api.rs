use super::model::Response;
use crate::utils::{constants::BODY, Res};
use reqwest::{header::HeaderMap, Client};

pub async fn get_code(client: &Client) -> Res<String> {
    let mut headers = HeaderMap::new();
    headers.append("accept", "*/*".parse().unwrap());
    headers.append("accept-language", "es-ES,es;q=0.9".parse().unwrap());
    headers.append("content-type", "application/json".parse().unwrap());
    headers.append(
        "sec-ch-ua",
        "\"Opera GX\";v=\"105\", \"Chromium\";v=\"119\", \"Not?A_Brand\";v=\"24\""
            .parse()
            .unwrap(),
    );
    headers.append("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.append("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    headers.append("sec-fetch-dest", "empty".parse().unwrap());
    headers.append("sec-fetch-mode", "cors".parse().unwrap());
    headers.append("sec-fetch-site", "cross-site".parse().unwrap());
    headers.append("Referer", "https://www.opera.com/".parse().unwrap());
    headers.append(
        "Referrer-Policy",
        "strict-origin-when-cross-origin".parse().unwrap(),
    );
    headers.append("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36 OPR/105.0.0.0".parse().unwrap());
    headers.append("Origin", "https://www.opera.com".parse().unwrap());

    let req = client
        .post("https://api.discord.gx.games/v1/direct-fulfillment")
        .headers(headers)
        .body(BODY)
        .send()
        .await?
        .text()
        .await?;

    let res: Response = serde_json::from_str(&req)?;
    
    Ok(format!(
        "https://discord.com/billing/partner-promotions/1180231712274387115/{}",
        res.token
    ))
}
