use futures::{stream, StreamExt};
use reqwest::{self, header};

mod html_fmter;
use html_fmter::fmt_html;

const CONCURRENT_REQUESTS: usize = 15;

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

    let mut urls = Vec::with_capacity((end_ch - start_ch + 1) as usize);
    let mut combined_txt = String::new();
    for chapter in start_ch..=end_ch {
        let request_url = format!(
            "https://ncode.syosetu.com/{ncode}/{chapter}/",
            ncode = ncode.trim()
        );

        urls.push(request_url);

        combined_txt.push_str(&format!("%%%{}&&&", chapter - 1));

        // let res = client.get(&request_url).send().await?.text().await?;

        // combined_txt.push_str(&fmt_html(&res));
    }

    let fetches = stream::iter(urls)
        .enumerate()
        .map(|(idx, url)| {
            let client = &client;
            async move {
                match client.get(&url).send().await {
                    Ok(resp) => match resp.text().await {
                        Ok(text) => {
                            return (idx, fmt_html(&text));
                        }
                        Err(_) => println!("ERROR reading {}", url),
                    },
                    Err(_) => println!("ERROR downloading {}", url),
                }
                (idx, String::from("failed"))
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS)
        .collect::<Vec<(usize, String)>>();

    let results = fetches.await;

    results.iter().for_each(|(idx, content)| {
        combined_txt = combined_txt.replace(&format!("%%%{}&&&", idx), content);
    });

    Ok(combined_txt)
}
