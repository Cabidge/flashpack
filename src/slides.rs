use line_span::LineSpanExt;

/// A way to break up a markdown string into multiple
/// sections by "Thematic breaks" as per the CommonMark specification.
pub struct ThematicBreaks<'a> {
    source: Option<&'a str>,
}

impl<'a> ThematicBreaks<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: Some(source),
        }
    }
}

impl<'a> Iterator for ThematicBreaks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let source = self.source.as_mut()?;

        // if this is false, the hyphens are a part of a "Setext heading"
        let mut can_break = true;
        let mut section_length = 0;
        let mut break_size = 0;
        let mut has_next = false;
        for span in source.line_spans() {
            let span_size = span.range_with_ending().len();
            section_length += span_size;
            let Some(line) = strip_indent(&span) else {
                can_break = true;
                continue;
            };

            // if the first non-whitespaace is alphanumeric, the next line can't
            // be a thematic break
            if line.chars().next().is_some_and(|ch| ch.is_alphanumeric()) {
                can_break = false;
                continue;
            }

            // if this line cannot be a break, the next one might be
            if !can_break {
                can_break = true;
                continue;
            }

            let mut chars = line.chars();
            // we need at 3 hyphens for it to be a thematic break
            if chars.by_ref().take_while(|&ch| ch == '-').count() < 3 {
                continue;
            }

            // if any characters after the hyphens are not whitespaces,
            // this can't be a thematic break
            if chars.any(|ch| !ch.is_whitespace()) {
                continue;
            }

            // if we pass all of the previous checks, we must be on a thematic break
            break_size = span_size;
            has_next = !span.ending_str().is_empty();
            break;
        }

        let (section, rest) = source.split_at(section_length);
        // trim off the thematic break lines from the result
        let section = &section[..(section_length - break_size)];

        if has_next {
            *source = rest;
        } else {
            self.source = None;
        }

        Some(section)
    }
}

/// Unindents a string and return the result,
/// but returns None if it could be interpreted as an "Indented code block"
fn strip_indent(source: &str) -> Option<&str> {
    let trimmed = source.trim_start();
    let indentation = &source[..(source.len() - trimmed.len())];

    (indentation.len() < 4 && indentation.chars().all(|ch| ch == ' ')).then_some(trimmed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_nothing() {
        let input = "12 3456     ";
        let stripped = strip_indent(input);
        assert_eq!(stripped, Some(input));
    }

    #[test]
    fn strip_spaces() {
        assert_eq!(strip_indent(" foo"), Some("foo"));
        assert_eq!(strip_indent("  foo"), Some("foo"));
        assert_eq!(strip_indent("   foo"), Some("foo"));
        assert_eq!(strip_indent("    foo"), None);
    }

    #[test]
    fn strip_tabs() {
        assert_eq!(strip_indent("\tfoo"), None);
    }

    #[test]
    fn thematic_empty() {
        let mut sections = ThematicBreaks::new("");
        assert_eq!(sections.next(), Some(""));
    }
}
