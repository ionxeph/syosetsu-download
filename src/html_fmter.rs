use select::document::Document;
use select::predicate::*;

pub fn fmt_html(html: &str) -> String {
    let document = Document::from(html);
    let mut result = String::new();
    // add header
    for node in document.find(Attr("class", "novel_subtitle")) {
        result.push_str(node.inner_html().trim());
    }
    result.push('\n');
    result.push('\n');
    // add the rest
    for node in document.find(Attr("id", "novel_honbun").descendant(Name("p"))) {
        let inner_html = node.inner_html();
        let trimmed = inner_html.trim();
        // handle br
        if trimmed == "<br>" {
            result.push('\n');
            continue;
        }
        // handle furigana
        if trimmed.contains("<ruby>") {
            let handled = handle_furigana(trimmed.to_owned());
            // let handled = handle_furigana(trimmed)
            //     .unwrap_or_else(|_| panic!("Handling furigana failed: {}", trimmed));
            result.push_str("　　");
            result.push_str(&handled);
            result.push('\n');
            continue;
        }
        result.push_str("　　");
        result.push_str(trimmed);
        result.push('\n');
    }
    result.push('\n');
    result.push_str("================");
    result.push('\n');
    result.push('\n');
    result
}

fn handle_furigana(unhandled: String) -> String {
    unhandled
        .trim()
        .to_owned()
        .replace("<ruby>", "")
        .replace("</ruby>", "")
        .replace("<rb>", "")
        .replace("</rb>", "")
        .replace("<rp>", "")
        .replace("</rp>", "")
        .replace("<rt>", "")
        .replace("</rt>", "")
}
