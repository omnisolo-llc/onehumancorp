
pub mod dom_scraper {
    //! # DOM Scraper
    //!
    //! A high-performance HTML parser and text extractor designed to feed clean,
    //! minified text into the agent context window to prevent token exhaustion.

    use std::collections::HashSet;

    /// Configuration for the DOM Scraper.
    pub struct ScraperConfig {
        pub block_tags: HashSet<String>,
        pub inline_tags: HashSet<String>,
        pub ignore_tags: HashSet<String>,
        pub max_depth: usize,
    }

    impl Default for ScraperConfig {
        fn default() -> Self {
            let mut block = HashSet::new();
            for t in &["div", "p", "h1", "h2", "h3", "h4", "h5", "h6", "section", "article", "header", "footer", "main", "aside", "nav", "ul", "ol", "li", "table", "tr", "td", "th", "blockquote", "pre", "form", "fieldset", "figure", "figcaption"] {
                block.insert(t.to_string());
            }

            let mut inline = HashSet::new();
            for t in &["span", "a", "strong", "em", "b", "i", "u", "s", "q", "abbr", "cite", "code", "kbd", "samp", "var", "mark", "sub", "sup", "small", "big", "time", "data", "ruby"] {
                inline.insert(t.to_string());
            }

            let mut ignore = HashSet::new();
            for t in &["script", "style", "style", "noscript", "iframe", "svg", "canvas", "video", "audio", "embed", "object", "param", "source", "track", "map", "area", "math", "applet", "frame", "frameset", "noframes"] {
                ignore.insert(t.to_string());
            }

            Self {
                block_tags: block,
                inline_tags: inline,
                ignore_tags: ignore,
                max_depth: 50,
            }
        }
    }

    pub struct HtmlNode {
        pub tag_name: String,
        pub attributes: std::collections::HashMap<String, String>,
        pub children: Vec<HtmlNode>,
        pub text_content: Option<String>,
    }

    pub struct DomScraper {
        config: ScraperConfig,
    }

    impl DomScraper {
        pub fn new() -> Self {
            Self {
                config: ScraperConfig::default(),
            }
        }

        pub fn extract_text(&self, html: &str) -> String {
            let mut result = String::new();
            let mut in_tag = false;
            let mut current_tag = String::new();
            let mut ignore_depth = 0;

            let chars: Vec<char> = html.chars().collect();
            let mut i = 0;

            while i < chars.len() {
                let c = chars[i];
                if c == '<' {
                    if i + 1 < chars.len() && chars[i+1] == '/' {
                        // Closing tag
                        let mut j = i + 2;
                        let mut tag = String::new();
                        while j < chars.len() && chars[j] != '>' {
                            if !chars[j].is_whitespace() {
                                tag.push(chars[j]);
                            }
                            j += 1;
                        }
                        let tag_lower = tag.to_lowercase();
                        if self.config.ignore_tags.contains(&tag_lower) {
                            if ignore_depth > 0 { ignore_depth -= 1; }
                        } else if ignore_depth == 0 && self.config.block_tags.contains(&tag_lower) {
                            result.push('\n');
                        }
                        i = j;
                    } else if i + 1 < chars.len() && chars[i+1] == '!' {
                        // Comment or doctype
                        let mut j = i + 2;
                        while j < chars.len() && chars[j] != '>' {
                            j += 1;
                        }
                        i = j;
                    } else {
                        // Opening tag
                        let mut j = i + 1;
                        let mut tag = String::new();
                        while j < chars.len() && chars[j] != '>' && !chars[j].is_whitespace() {
                            tag.push(chars[j]);
                            j += 1;
                        }
                        let tag_lower = tag.to_lowercase();

                        // skip attributes
                        while j < chars.len() && chars[j] != '>' {
                            j += 1;
                        }

                        if self.config.ignore_tags.contains(&tag_lower) {
                            ignore_depth += 1;
                        } else if ignore_depth == 0 && self.config.block_tags.contains(&tag_lower) {
                            result.push('\n');
                        }
                        i = j;
                    }
                } else if ignore_depth == 0 {
                    result.push(c);
                }
                i += 1;
            }

            // Cleanup whitespace
            let mut final_res = String::new();
            let mut last_was_space = true;
            for c in result.chars() {
                if c.is_whitespace() {
                    if !last_was_space {
                        if c == '\n' {
                            final_res.push('\n');
                            last_was_space = true;
                        } else {
                            final_res.push(' ');
                            last_was_space = true;
                        }
                    }
                } else {
                    final_res.push(c);
                    last_was_space = false;
                }
            }

            final_res.trim().to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom_scraper::dom_scraper::DomScraper;

    #[test]
    fn test_extract_text_basic() {
        let scraper = DomScraper::new();
        let html = "<html><body><h1>Title</h1><p>Some text</p></body></html>";
        let res = scraper.extract_text(html);
        assert_eq!(res, "Title\nSome text");
    }

    #[test]
    fn test_extract_text_with_inline() {
        let scraper = DomScraper::new();
        let html = "<p>Hello <b>bold</b> world</p>";
        let res = scraper.extract_text(html);
        assert_eq!(res, "Hello bold world");
    }

    #[test]
    fn test_extract_text_with_ignore() {
        let scraper = DomScraper::new();
        let html = "<div><script>alert(1);</script>Text</div>";
        let res = scraper.extract_text(html);
        assert_eq!(res, "Text");
    }
}
