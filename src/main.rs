use reqwest::{self, header};
use std::fs;

mod html_fmter;
use html_fmter::fmt_html;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("Mozilla/5.0...."),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // let request_url = format!(
    //     "https://ncode.syosetu.com/txtdownload/dlstart/ncode/{ncode}/?no={chapter}&hankaku=0&code=utf-8&kaigyo=crlf",
    //     ncode = "1783912",
    //     chapter = 1
    // );
    let request_url = String::from("https://ncode.syosetu.com/n4090gw/930/");

    let res = client.get(&request_url).send().await?.text().await?;

    let fmt_res = fmt_html(&res);

    fs::write("output/test.txt", fmt_res).expect("Unable to write file");

    Ok(())
}
