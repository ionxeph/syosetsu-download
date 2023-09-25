use reqwest::{self, header};
use std::{env, fs, io};

mod html_fmter;
use html_fmter::fmt_html;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
        env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
    );
    println!("Please input ncode.");
    let mut ncode = String::new();
    io::stdin()
        .read_line(&mut ncode)
        .expect("Failed to read ncode");

    println!("Please enter start chapter.");
    let mut start_ch_str = String::new();
    io::stdin()
        .read_line(&mut start_ch_str)
        .expect("Failed to read start chapter");
    let start_ch: i32 = start_ch_str
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("Start chapter number parse failed: {}", start_ch_str));

    println!("Please enter end chapter.");
    let mut end_ch_str = String::new();
    io::stdin()
        .read_line(&mut end_ch_str)
        .expect("Failed to read end chapter");
    let end_ch: i32 = end_ch_str
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("End chapter number parse failed: {}", end_ch_str));

    println!("Please input file title.");
    let mut file_title = String::new();
    io::stdin()
        .read_line(&mut file_title)
        .expect("Failed to read file title");

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

    let trimmed_file_name = &file_title.trim();
    println!(
        "{}",
        &format!(
            "writing to output/{file_title}.txt",
            file_title = trimmed_file_name
        )
    );
    fs::create_dir_all("output")?;
    fs::write(
        format!("output/{file_title}.txt", file_title = trimmed_file_name),
        combined_txt,
    )
    .expect("Unable to write file");

    std::thread::sleep(std::time::Duration::from_secs(5));
    Ok(())
}
