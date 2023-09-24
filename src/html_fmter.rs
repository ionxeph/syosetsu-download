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
        // ignore br
        if node.inner_html().trim() == "<br>" {
            continue;
        }
        result.push_str(node.inner_html().trim());
        result.push('\n');
    }
    result.push('\n');
    result.push_str("================");
    result
}
