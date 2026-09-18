//! Interprets the closed subset of HTML tags RF-13.1 lists (AD-22), inside
//! Markdown that `pulldown_cmark` already recognized as raw HTML text but
//! left uninterpreted. Owns no knowledge of GPUI, colors, sizes, or fonts —
//! same rule as `markdown`, whose job this is a part of.
//!
//! `pulldown_cmark` hands over one tag per event (`<b>`, then `</b>`, with
//! ordinary `Text` events for whatever is in between), never a whole
//! subtree at once: parsing one tag at a time, here, matches that shape
//! instead of fighting it with a tree builder this scope doesn't need.

/// One HTML start or end tag, e.g. `<a href="https://x.dev">` or `</a>`.
/// Attribute values are unescaped as written: RF-13.1 doesn't ask for HTML
/// entity decoding, and no test document needs it.
pub struct Tag {
    /// Lowercased, without the enclosing `<`/`>`, `/`, or attributes.
    pub name: String,
    pub closing: bool,
    /// `<br/>`, not `<br>` — RF-13.1's tags that use this (`img`, `br`,
    /// `hr`) don't require it, so it's tracked but nothing currently reads it.
    #[allow(dead_code)]
    pub self_closing: bool,
    /// In document order; duplicate names keep only the first (attributes
    /// this module reads are only ever looked up once, by name).
    pub attrs: Vec<(String, String)>,
}

impl Tag {
    pub fn attr(&self, name: &str) -> Option<&str> {
        self.attrs.iter().find(|(n, _)| n == name).map(|(_, v)| v.as_str())
    }
}

/// Parses one raw HTML tag as `pulldown_cmark` hands it over (the full text
/// of a single `Event::Html`/`Event::InlineHtml`). Returns `None` for
/// anything that isn't a well-formed `<name ...>` or `</name>` — a comment,
/// a doctype, a stray `<` in text CommonMark treated as HTML by mistake —
/// so the caller can fall back to showing nothing for it (RF-13.1: an
/// unlisted tag shows no markup, and an unparseable one is no different).
pub fn parse_tag(raw: &str) -> Option<Tag> {
    let raw = raw.trim();
    let inner = raw.strip_prefix('<')?.strip_suffix('>')?;
    if inner.starts_with('!') {
        return None; // comment or doctype, not a tag
    }

    let closing = inner.starts_with('/');
    let inner = if closing { &inner[1..] } else { inner };
    let self_closing = inner.ends_with('/');
    let inner = if self_closing { inner[..inner.len() - 1].trim_end() } else { inner };

    let (name, rest) = match inner.find(|c: char| c.is_whitespace()) {
        Some(i) => (&inner[..i], &inner[i..]),
        None => (inner, ""),
    };
    if name.is_empty() {
        return None;
    }

    Some(Tag { name: name.to_lowercase(), closing, self_closing, attrs: parse_attrs(rest) })
}

/// Parses `name="value"` / `name='value'` pairs. An attribute without a
/// quoted value (`disabled`, or a stray malformed fragment) is skipped, not
/// an error: RF-13.1's tags are only ever read for a handful of known
/// attribute names, so a value this can't extract is the same as an
/// absent one to every caller.
fn parse_attrs(s: &str) -> Vec<(String, String)> {
    let bytes = s.as_bytes();
    let mut attrs = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        let name_start = i;
        while i < bytes.len() && bytes[i] != b'=' && !bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        let name = &s[name_start..i];
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }

        if name.is_empty() {
            if i >= bytes.len() {
                break;
            }
            i += 1; // skip one unrecognized byte and keep scanning
            continue;
        }

        if i < bytes.len() && bytes[i] == b'=' {
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            if i < bytes.len() && (bytes[i] == b'"' || bytes[i] == b'\'') {
                let quote = bytes[i];
                i += 1;
                let value_start = i;
                while i < bytes.len() && bytes[i] != quote {
                    i += 1;
                }
                attrs.push((name.to_lowercase(), s[value_start..i].to_string()));
                if i < bytes.len() {
                    i += 1; // skip closing quote
                }
            }
            // An unquoted or missing value: attribute dropped, per the doc
            // comment above.
        }
    }

    attrs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_simple_opening_and_closing_tag() {
        let open = parse_tag("<b>").unwrap();
        assert_eq!(open.name, "b");
        assert!(!open.closing);

        let close = parse_tag("</b>").unwrap();
        assert_eq!(close.name, "b");
        assert!(close.closing);
    }

    #[test]
    fn parses_attributes_with_either_quote_style() {
        let tag = parse_tag(r#"<a href="https://x.dev" title='ejemplo'>"#).unwrap();
        assert_eq!(tag.name, "a");
        assert_eq!(tag.attr("href"), Some("https://x.dev"));
        assert_eq!(tag.attr("title"), Some("ejemplo"));
    }

    #[test]
    fn recognizes_self_closing_tags() {
        let tag = parse_tag(r#"<img src="x.png" alt="y" />"#).unwrap();
        assert!(tag.self_closing);
        assert_eq!(tag.attr("src"), Some("x.png"));
        assert_eq!(tag.attr("alt"), Some("y"));
    }

    #[test]
    fn tag_name_is_lowercased() {
        assert_eq!(parse_tag("<B>").unwrap().name, "b");
        assert_eq!(parse_tag("<DIV ALIGN=\"center\">").unwrap().name, "div");
    }

    #[test]
    fn a_comment_is_not_a_tag() {
        assert!(parse_tag("<!-- nota -->").is_none());
    }
}
