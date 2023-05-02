use pulldown_cmark::{CodeBlockKind, Event, Tag};

pub struct Parser<'input, 'callback> {
    events: pulldown_cmark::Parser<'input, 'callback>,
}

impl<'a, 'b> Iterator for Parser<'a, 'b> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let event = self.events.next()?;

        match &event {
            Event::Code(code) => {
                let Some(code) = code.strip_prefix('$').and_then(|s| s.strip_suffix('$')) else {
                    return Some(event);
                };

                let html = render_math(&code, MathDisplay::Inline);
                Some(Event::Html(html.into()))
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) if *lang == "math".into() => {
                let code = self.merge_code_block("math");

                let html = render_math(&code, MathDisplay::Block);
                Some(Event::Html(html.into()))
            }
            _ => Some(event),
        }
    }
}

impl<'a> Parser<'a, '_> {
    pub fn new(markdown: &'a str) -> Self {
        let events = pulldown_cmark::Parser::new(markdown);

        Self { events }
    }

    fn merge_code_block(&mut self, block_lang: &str) -> String {
        let mut inner = String::new();

        for event in &mut self.events {
            match event {
                Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(lang)))
                    if lang == block_lang.into() =>
                {
                    break;
                }
                Event::Text(text) => {
                    inner += &text;
                }
                _ => (),
            }
        }

        inner
    }
}

enum MathDisplay {
    Inline,
    Block,
}

fn render_math(source: &str, display: MathDisplay) -> String {
    let tex = asciimath_to_tex(source);

    let opts = katex::Opts::builder()
        .display_mode(matches!(display, MathDisplay::Block))
        .build()
        .expect("Option builder should not fail");

    katex::render_with_opts(&tex, &opts).unwrap_or_default()
}

fn asciimath_to_tex(source: &str) -> String {
    let mut script = js_sandbox::Script::from_string(include_str!("asciimath_tex.js"))
        .expect("Script should not fail");

    script
        .call("AMTparseAMtoTeX", &source)
        .expect("Function should not fail")
}
