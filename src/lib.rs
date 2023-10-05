use reqwest::{self, header};

mod html_fmter;
use html_fmter::fmt_html;

pub async fn request_data(
    start_ch: i32,
    end_ch: i32,
    ncode: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("Mozilla/5.0...."),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let mut combined_txt = String::new();
    for chapter in start_ch..=end_ch {
        let request_url = format!(
            "https://ncode.syosetu.com/{ncode}/{chapter}/",
            ncode = ncode.trim()
        );

        let res = client.get(&request_url).send().await?.text().await?;

        combined_txt.push_str(&fmt_html(&res));
    }

    Ok(combined_txt)
}
