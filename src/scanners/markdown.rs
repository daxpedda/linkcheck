use crate::codespan::Span;
use pulldown_cmark::{Event, Options, Parser, Tag};

/// A scanner that uses [`pulldown_cmark`] to extract all links from markdown.
///
/// # Examples
///
/// ```rust
/// # use linkcheck::codespan::Span;
/// let src = "This is a [link](https://example.com/) and an ![Image](img.png)";
///
/// let got: Vec<_> = linkcheck::scanners::markdown(src).collect();
///
/// assert_eq!(got.len(), 2);
/// let (href, span) = &got[0];
/// assert_eq!(href, "https://example.com/");
/// assert_eq!(*span, Span::new(10, 38));
/// ```
pub fn markdown(src: &str) -> impl Iterator<Item = (String, Span)> + '_ {
    markdown_with_broken_link_callback(src, &|_, _| None)
}

/// A scanner that uses [`pulldown_cmark`] to extract all links from markdown,
/// using the supplied callback to try and fix broken links.
pub fn markdown_with_broken_link_callback<'a, F>(
    src: &'a str,
    on_broken_link: &'a F,
) -> impl Iterator<Item = (String, Span)> + 'a
where
    F: Fn(&str, &str) -> Option<(String, String)>,
{
    Parser::new_with_broken_link_callback(
        src,
        Options::ENABLE_FOOTNOTES,
        Some(on_broken_link),
    )
    .into_offset_iter()
    .filter_map(|(event, range)| match event {
        Event::Start(Tag::Link(_, dest, _))
        | Event::Start(Tag::Image(_, dest, _)) => Some((
            dest.to_string(),
            Span::new(range.start as u32, range.end as u32),
        )),
        _ => None,
    })
}
