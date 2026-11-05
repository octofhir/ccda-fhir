//! CDA narrative (`section/text`) ⇄ FHIR narrative (XHTML `div`).
//!
//! Follows "Narrative Text" of C-CDA on FHIR (mapping guidance): `content` ⇄ `span`,
//! `paragraph` ⇄ `p`, `list` ⇄ `ul`/`ol` by `@listType`, `item` ⇄ `li`, `linkHtml` ⇄ `a`,
//! `renderMultiMedia` ⇄ `img`, `@ID` ⇄ `@id`, `@styleCode` ⇄ `@class` (with `style` for the
//! renderings CDA defines: `Bold`, `Italics`, `Underline`, `Emphasis`), tables as they are.
//! A list's caption, which XHTML does not allow inside a list, goes before it as a paragraph of
//! class `caption`, and comes back.
//!
//! What has no counterpart is kept as text and reported.

use roxmltree::{Document, Node, ParsingOptions};

const XHTML: &str = "http://www.w3.org/1999/xhtml";

/// What a conversion could not carry over.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NarrativeLoss {
    pub message: String,
}

/// A CDA `section/text` (the markup the XML reader keeps) as a FHIR narrative `div`.
pub fn cda_to_xhtml(markup: &str) -> Result<(String, Vec<NarrativeLoss>), String> {
    let doc = parse(markup)?;
    let root = doc.root_element();
    let mut out = String::new();
    let mut losses = Vec::new();
    out.push_str(&format!("<div xmlns=\"{XHTML}\""));
    attributes_to_xhtml(root, &mut out, &mut losses);
    out.push('>');
    children_to_xhtml(root, &mut out, &mut losses);
    out.push_str("</div>");
    Ok((out, losses))
}

/// A FHIR narrative `div` as a CDA `section/text`, in the CDA namespace of the document it goes
/// into (written without a prefix).
pub fn xhtml_to_cda(div: &str) -> Result<(String, Vec<NarrativeLoss>), String> {
    let doc = parse(div)?;
    let root = doc.root_element();
    let mut out = String::from("<text");
    let mut losses = Vec::new();
    attributes_to_cda(root, &mut out, &mut losses);
    out.push('>');
    children_to_cda(root, &mut out, &mut losses);
    out.push_str("</text>");
    Ok((out, losses))
}

/// The text of the narrative element whose `ID` is `id` (a `reference/@value` without `#`),
/// whitespace collapsed, as C-CDA on FHIR says originalText references resolve.
pub fn text_of_id(markup: &str, id: &str) -> Option<String> {
    let doc = parse(markup).ok()?;
    let node = doc
        .descendants()
        .find(|n| n.is_element() && n.attribute("ID") == Some(id))?;
    Some(collapse(&node_text(node)))
}

/// Every element with an `ID` in a narrative, with its text, whitespace collapsed.
pub fn texts_by_id(markup: &str) -> Vec<(String, String)> {
    let Ok(doc) = parse(markup) else {
        return Vec::new();
    };
    doc.descendants()
        .filter(|n| n.is_element())
        .filter_map(|n| Some((n.attribute("ID")?.to_owned(), collapse(&node_text(n)))))
        .collect()
}

/// The narrative element whose `ID` is `id`, as a FHIR narrative `div` (a resource's text).
pub fn xhtml_of_id(markup: &str, id: &str) -> Option<String> {
    let doc = parse(markup).ok()?;
    let node = doc
        .descendants()
        .find(|n| n.is_element() && n.attribute("ID") == Some(id))?;
    let mut out = format!("<div xmlns=\"{XHTML}\">");
    let mut losses = Vec::new();
    match node.tag_name().name() {
        // A row or cell does not stand on its own: the text of it, in a paragraph.
        "tr" | "td" | "th" | "tbody" | "thead" | "tfoot" | "col" | "colgroup" | "item" => {
            out.push_str("<p>");
            escape_text(&collapse(&node_text(node)), &mut out);
            out.push_str("</p>");
        }
        _ => element_to_xhtml(node, &mut out, &mut losses),
    }
    out.push_str("</div>");
    Some(out)
}

/// The text of a markup document (XHTML, XML), whitespace collapsed; `None` when it does not
/// parse.
pub fn plain_text(markup: &str) -> Option<String> {
    let doc = parse(markup).ok()?;
    Some(collapse(&node_text(doc.root_element())))
}

/// `text` escaped for XML content.
pub fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn parse(text: &str) -> Result<Document<'_>, String> {
    let options = ParsingOptions {
        allow_dtd: false,
        ..ParsingOptions::default()
    };
    Document::parse_with_options(text, options).map_err(|e| format!("narrative: {e}"))
}

fn node_text(node: Node<'_, '_>) -> String {
    node.descendants()
        .filter(|n| n.is_text())
        .filter_map(|n| n.text())
        .collect::<Vec<_>>()
        .join(" ")
}

fn collapse(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

// ---- CDA → XHTML ---------------------------------------------------------------------------

fn children_to_xhtml(node: Node<'_, '_>, out: &mut String, losses: &mut Vec<NarrativeLoss>) {
    for child in node.children() {
        if child.is_text() {
            escape_text(child.text().unwrap_or_default(), out);
        } else if child.is_element() {
            element_to_xhtml(child, out, losses);
        }
    }
}

fn element_to_xhtml(node: Node<'_, '_>, out: &mut String, losses: &mut Vec<NarrativeLoss>) {
    let name = node.tag_name().name();
    let tag = match name {
        "content" => "span",
        "paragraph" => "p",
        "list" => {
            if node.attribute("listType") == Some("ordered") {
                "ol"
            } else {
                "ul"
            }
        }
        "item" => "li",
        "linkHtml" => "a",
        "renderMultiMedia" => "img",
        "footnote" | "footnoteRef" => "span",
        "br" | "sub" | "sup" | "table" | "thead" | "tbody" | "tfoot" | "colgroup" | "col"
        | "tr" | "th" | "td" | "caption" => name,
        other => {
            losses.push(NarrativeLoss {
                message: format!(
                    "narrative element `{other}` has no XHTML counterpart; its text is kept"
                ),
            });
            "span"
        }
    };
    // A list's caption cannot be inside the list in XHTML: it goes before it.
    if name == "list"
        && let Some(caption) = node
            .children()
            .find(|c| c.is_element() && c.tag_name().name() == "caption")
    {
        out.push_str("<p class=\"caption\">");
        children_to_xhtml(caption, out, losses);
        out.push_str("</p>");
    }
    out.push('<');
    out.push_str(tag);
    attributes_to_xhtml(node, out, losses);
    if matches!(name, "footnote" | "footnoteRef") {
        out.push_str(&format!(" data-cda=\"{name}\""));
    }
    if name == "renderMultiMedia" {
        if let Some(object) = node.attribute("referencedObject") {
            out.push_str(" src=\"#");
            escape_attribute(object, out);
            out.push('"');
        }
        out.push_str(" alt=\"\"/>");
        return;
    }
    let has_content = node.children().any(|c| {
        c.is_text() || (c.is_element() && !(name == "list" && c.tag_name().name() == "caption"))
    });
    if !has_content && matches!(tag, "br" | "col") {
        out.push_str("/>");
        return;
    }
    out.push('>');
    for child in node.children() {
        if child.is_text() {
            escape_text(child.text().unwrap_or_default(), out);
        } else if child.is_element() {
            if name == "list" && child.tag_name().name() == "caption" {
                continue;
            }
            element_to_xhtml(child, out, losses);
        }
    }
    out.push_str("</");
    out.push_str(tag);
    out.push('>');
}

/// CDA renderings with a standard meaning, as CSS.
const STYLES: &[(&str, &str)] = &[
    ("Bold", "font-weight: bold"),
    ("Italics", "font-style: italic"),
    ("Underline", "text-decoration: underline"),
    ("Emphasis", "font-style: italic"),
];

fn attributes_to_xhtml(node: Node<'_, '_>, out: &mut String, losses: &mut Vec<NarrativeLoss>) {
    for attribute in node.attributes() {
        let (name, value) = (attribute.name(), attribute.value());
        let target = match (node.tag_name().name(), name) {
            (_, "ID") => "id",
            (_, "language") => "lang",
            (_, "styleCode") => {
                out.push_str(" class=\"");
                escape_attribute(value, out);
                out.push('"');
                let styles: Vec<&str> = value
                    .split_whitespace()
                    .filter_map(|code| STYLES.iter().find(|(c, _)| *c == code).map(|(_, s)| *s))
                    .collect();
                if !styles.is_empty() {
                    out.push_str(" style=\"");
                    escape_attribute(&styles.join("; "), out);
                    out.push('"');
                }
                continue;
            }
            ("list", "listType") | ("renderMultiMedia", "referencedObject") => continue,
            ("linkHtml", "href" | "name" | "title" | "rel" | "rev") => name,
            (
                "table" | "thead" | "tbody" | "tfoot" | "colgroup" | "col" | "tr" | "th" | "td",
                "border" | "cellpadding" | "cellspacing" | "width" | "summary" | "frame" | "rules"
                | "align" | "valign" | "char" | "charoff" | "span" | "abbr" | "axis" | "headers"
                | "scope" | "rowspan" | "colspan",
            ) => name,
            (_, other) => {
                losses.push(NarrativeLoss {
                    message: format!("narrative attribute `{other}` is not carried over"),
                });
                continue;
            }
        };
        out.push(' ');
        out.push_str(target);
        out.push_str("=\"");
        escape_attribute(value, out);
        out.push('"');
    }
}

// ---- XHTML → CDA ---------------------------------------------------------------------------

fn children_to_cda(node: Node<'_, '_>, out: &mut String, losses: &mut Vec<NarrativeLoss>) {
    for child in node.children() {
        if child.is_text() {
            escape_text(child.text().unwrap_or_default(), out);
            continue;
        }
        if !child.is_element() {
            continue;
        }
        // A caption paragraph right before a list is the list's caption.
        if child.tag_name().name() == "p"
            && child.attribute("class") == Some("caption")
            && let Some(next) = next_element(child)
            && matches!(next.tag_name().name(), "ul" | "ol")
        {
            continue;
        }
        element_to_cda(child, out, losses);
    }
}

fn next_element<'a, 'input>(node: Node<'a, 'input>) -> Option<Node<'a, 'input>> {
    let mut next = node.next_sibling();
    while let Some(n) = next {
        if n.is_element() {
            return Some(n);
        }
        if n.is_text() && !n.text().unwrap_or_default().trim().is_empty() {
            return None;
        }
        next = n.next_sibling();
    }
    None
}

fn element_to_cda(node: Node<'_, '_>, out: &mut String, losses: &mut Vec<NarrativeLoss>) {
    let name = node.tag_name().name();
    let (tag, extra_style) = match name {
        "span" => match node.attribute("data-cda") {
            Some(cda @ ("footnote" | "footnoteRef")) => (cda, None),
            _ => ("content", None),
        },
        "p" | "div" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "blockquote" | "pre" => {
            ("paragraph", name.starts_with('h').then_some("Bold"))
        }
        "ul" | "ol" => ("list", None),
        "li" => ("item", None),
        "a" => ("linkHtml", None),
        "img" => ("renderMultiMedia", None),
        "b" | "strong" => ("content", Some("Bold")),
        "i" | "em" => ("content", Some("Italics")),
        "u" => ("content", Some("Underline")),
        "br" | "sub" | "sup" | "table" | "thead" | "tbody" | "tfoot" | "colgroup" | "col"
        | "tr" | "th" | "td" | "caption" => (name, None),
        other => {
            losses.push(NarrativeLoss {
                message: format!(
                    "XHTML element `{other}` has no CDA narrative counterpart; its text is kept"
                ),
            });
            ("content", None)
        }
    };
    out.push('<');
    out.push_str(tag);
    if name == "ol" {
        out.push_str(" listType=\"ordered\"");
    }
    if name == "img" {
        if let Some(src) = node.attribute("src").and_then(|s| s.strip_prefix('#')) {
            out.push_str(" referencedObject=\"");
            escape_attribute(src, out);
            out.push('"');
        } else {
            losses.push(NarrativeLoss {
                message: "an image that is not a reference to a multimedia entry is left out"
                    .into(),
            });
        }
        attributes_to_cda(node, out, losses);
        out.push_str("/>");
        return;
    }
    attributes_to_cda_with(node, out, losses, extra_style);
    let has_content = node.children().any(|c| c.is_text() || c.is_element());
    if !has_content && matches!(tag, "br" | "col") {
        out.push_str("/>");
        return;
    }
    out.push('>');
    // The caption paragraph before this list.
    if matches!(name, "ul" | "ol")
        && let Some(previous) = previous_element(node)
        && previous.tag_name().name() == "p"
        && previous.attribute("class") == Some("caption")
    {
        out.push_str("<caption>");
        children_to_cda(previous, out, losses);
        out.push_str("</caption>");
    }
    children_to_cda(node, out, losses);
    out.push_str("</");
    out.push_str(tag);
    out.push('>');
}

fn previous_element<'a, 'input>(node: Node<'a, 'input>) -> Option<Node<'a, 'input>> {
    let mut previous = node.prev_sibling();
    while let Some(n) = previous {
        if n.is_element() {
            return Some(n);
        }
        if n.is_text() && !n.text().unwrap_or_default().trim().is_empty() {
            return None;
        }
        previous = n.prev_sibling();
    }
    None
}

fn attributes_to_cda(node: Node<'_, '_>, out: &mut String, losses: &mut Vec<NarrativeLoss>) {
    attributes_to_cda_with(node, out, losses, None);
}

fn attributes_to_cda_with(
    node: Node<'_, '_>,
    out: &mut String,
    losses: &mut Vec<NarrativeLoss>,
    extra_style: Option<&str>,
) {
    let mut style_codes: Vec<String> = extra_style.into_iter().map(str::to_owned).collect();
    for attribute in node.attributes() {
        let (name, value) = (attribute.name(), attribute.value());
        if attribute.namespace().is_some() && attribute.namespace() != Some(XHTML) {
            if name == "lang" {
                out.push_str(" language=\"");
                escape_attribute(value, out);
                out.push('"');
            }
            continue;
        }
        let target = match (node.tag_name().name(), name) {
            (_, "id") => "ID",
            (_, "lang") => "language",
            (_, "class") => {
                style_codes.extend(value.split_whitespace().map(str::to_owned));
                continue;
            }
            // The renderings `class` already carries; other CSS has no CDA form.
            (_, "style") => {
                for declaration in value.split(';').map(str::trim).filter(|d| !d.is_empty()) {
                    match STYLES
                        .iter()
                        .find(|(_, css)| css.replace(' ', "") == declaration.replace(' ', ""))
                    {
                        Some((code, _)) => style_codes.push((*code).to_owned()),
                        None => losses.push(NarrativeLoss {
                            message: format!("style `{declaration}` has no CDA narrative form"),
                        }),
                    }
                }
                continue;
            }
            (_, "data-cda") | ("img", "src" | "alt") => continue,
            ("a", "href" | "name" | "title" | "rel" | "rev") => name,
            (
                "table" | "thead" | "tbody" | "tfoot" | "colgroup" | "col" | "tr" | "th" | "td",
                "border" | "cellpadding" | "cellspacing" | "width" | "summary" | "frame" | "rules"
                | "align" | "valign" | "char" | "charoff" | "span" | "abbr" | "axis" | "headers"
                | "scope" | "rowspan" | "colspan",
            ) => name,
            (_, "xmlns") => continue,
            (_, other) => {
                losses.push(NarrativeLoss {
                    message: format!("XHTML attribute `{other}` has no CDA narrative form"),
                });
                continue;
            }
        };
        out.push(' ');
        out.push_str(target);
        out.push_str("=\"");
        escape_attribute(value, out);
        out.push('"');
    }
    let mut seen = std::collections::HashSet::new();
    style_codes.retain(|code| code != "caption" && seen.insert(code.clone()));
    if !style_codes.is_empty() {
        out.push_str(" styleCode=\"");
        escape_attribute(&style_codes.join(" "), out);
        out.push('"');
    }
}

fn escape_text(text: &str, out: &mut String) {
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(c),
        }
    }
}

fn escape_attribute(text: &str, out: &mut String) {
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CDA: &str = r#"<text><paragraph styleCode="Bold">Allergies</paragraph><list listType="ordered"><caption>Known</caption><item ID="a1">Penicillin &amp; <content styleCode="Italics">hives</content></item></list><table border="1"><thead><tr><th>Substance</th></tr></thead><tbody><tr ID="r1"><td>Codeine</td></tr></tbody></table><br/><renderMultiMedia referencedObject="img1"/><linkHtml href="http://example.org">link</linkHtml></text>"#;

    #[test]
    fn cda_to_xhtml_and_back() {
        let (xhtml, losses) = cda_to_xhtml(CDA).unwrap();
        assert!(losses.is_empty(), "{losses:?}");
        assert_eq!(
            xhtml,
            r##"<div xmlns="http://www.w3.org/1999/xhtml"><p class="Bold" style="font-weight: bold">Allergies</p><p class="caption">Known</p><ol><li id="a1">Penicillin &amp; <span class="Italics" style="font-style: italic">hives</span></li></ol><table border="1"><thead><tr><th>Substance</th></tr></thead><tbody><tr id="r1"><td>Codeine</td></tr></tbody></table><br/><img src="#img1" alt=""/><a href="http://example.org">link</a></div>"##
        );
        let (cda, losses) = xhtml_to_cda(&xhtml).unwrap();
        assert!(losses.is_empty(), "{losses:?}");
        assert_eq!(cda, CDA);
    }

    #[test]
    fn references_resolve_to_text() {
        assert_eq!(text_of_id(CDA, "a1").as_deref(), Some("Penicillin & hives"));
        assert_eq!(text_of_id(CDA, "nope"), None);
        assert_eq!(
            xhtml_of_id(CDA, "r1").as_deref(),
            Some(r#"<div xmlns="http://www.w3.org/1999/xhtml"><p>Codeine</p></div>"#)
        );
    }

    #[test]
    fn html_without_cda_form_is_reported() {
        let (cda, losses) = xhtml_to_cda(
            r#"<div xmlns="http://www.w3.org/1999/xhtml"><h2>Title</h2><p style="color: red">x</p><code>y</code></div>"#,
        )
        .unwrap();
        assert_eq!(
            cda,
            r#"<text><paragraph styleCode="Bold">Title</paragraph><paragraph>x</paragraph><content>y</content></text>"#
        );
        assert_eq!(losses.len(), 2);
    }
}
