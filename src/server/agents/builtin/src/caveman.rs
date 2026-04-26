use once_cell::sync::Lazy;
use regex::Regex;

/// Caveman compression mode — mirrors Go CavemanMode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CavemanMode {
    /// Full natural language (no compression).
    Off = 0,
    /// Remove filler/pleasantries, keep articles.
    Lite = 1,
    /// Drop articles, use fragments. Default for agent↔agent.
    Full = 2,
    /// Maximum compression with abbreviations and causal arrows.
    Ultra = 3,
}

/// Returns the system prompt suffix for the given caveman mode.
pub fn caveman_system_prompt(mode: CavemanMode) -> &'static str {
    match mode {
        CavemanMode::Off => "",
        CavemanMode::Lite => {
            "\n\n[COMM-MODE: lite] No filler/hedging. Keep articles + full sentences. Professional but tight."
        }
        CavemanMode::Full => {
            "\n\n[COMM-MODE: full] Terse caveman style. Drop articles. Fragments OK. Short synonyms. \
             Pattern: [thing] [action] [reason]. Technical terms exact. Code blocks unchanged."
        }
        CavemanMode::Ultra => {
            "\n\n[COMM-MODE: ultra] Max compression. Abbreviate (DB/auth/cfg/req/res/fn). \
             Strip conjunctions. Arrows for causality (X→Y). One word when one word enough."
        }
    }
}

// ── Regex patterns ────────────────────────────────────────────────────────────

static LITE_FILLER_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)\b(just|really|basically|actually|simply|certainly|of course|sure[,!]?|happy to[^.]*?\.|I'd be happy to[^.]*?\.|Let me[^.]*?\.|I'll help you[^.]*?\.|Great[,!]?|Absolutely[,!]?)\s*",
    )
    .unwrap()
});

static FULL_ARTICLE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)\b(a|an|the)\s+").unwrap());

static FULL_FILLER_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)\b(just|really|basically|actually|simply|certainly|of course|sure[,!]?|happy to|I would|I will|I'll|you might want to|you should consider|it is important to|please note that|in order to|so that you can)\s*",
    )
    .unwrap()
});

static FULL_HEDGE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)\b(I think|I believe|I suggest|I recommend|it seems like|it appears that|it looks like|might be|could be|probably|possibly|perhaps)\s+",
    )
    .unwrap()
});

static MULTI_SPACE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s{2,}").unwrap());

static ULTRA_ABBR_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)\b(database|authentication|configuration|implementation|function|request|response|because|therefore|which causes|which results in|leading to|resulting in)\b",
    )
    .unwrap()
});

static MULTI_ARROW_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(→\s*){2,}").unwrap());

static CODE_BLOCK_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)```[a-zA-Z]*\n.*?```|`[^`\n]+`").unwrap());

// ── Mode implementations ──────────────────────────────────────────────────────

fn apply_lite(s: &str) -> String {
    LITE_FILLER_RE.replace_all(s, " ").trim().to_string()
}

fn apply_full(s: &str) -> String {
    let s = apply_lite(s);
    let s = FULL_FILLER_RE.replace_all(&s, " ");
    let s = FULL_HEDGE_RE.replace_all(&s, "");
    let s = FULL_ARTICLE_RE.replace_all(&s, "");
    let s = MULTI_SPACE_RE.replace_all(&s, " ");
    s.trim().to_string()
}

fn apply_ultra(s: &str) -> String {
    let s = apply_full(s);
    let abbr_map = |m: &regex::Captures| -> String {
        match m.get(0).map(|m| m.as_str().to_lowercase()).as_deref() {
            Some("database") => "DB".to_string(),
            Some("authentication") => "auth".to_string(),
            Some("configuration") => "cfg".to_string(),
            Some("implementation") => "impl".to_string(),
            Some("function") => "fn".to_string(),
            Some("request") => "req".to_string(),
            Some("response") => "res".to_string(),
            Some("because")
            | Some("therefore")
            | Some("which causes")
            | Some("which results in")
            | Some("leading to")
            | Some("resulting in") => "→".to_string(),
            _ => m.get(0).map(|m| m.as_str().to_string()).unwrap_or_default(),
        }
    };
    let s = ULTRA_ABBR_RE.replace_all(&s, abbr_map);
    let s = MULTI_ARROW_RE.replace_all(&s, "→");
    s.trim().to_string()
}

struct TextPart {
    text: String,
    is_code: bool,
}

fn split_code_blocks(text: &str) -> Vec<TextPart> {
    let mut parts = Vec::new();
    let mut last = 0usize;
    for m in CODE_BLOCK_RE.find_iter(text) {
        if m.start() > last {
            parts.push(TextPart {
                text: text[last..m.start()].to_string(),
                is_code: false,
            });
        }
        parts.push(TextPart {
            text: text[m.start()..m.end()].to_string(),
            is_code: true,
        });
        last = m.end();
    }
    if last < text.len() {
        parts.push(TextPart {
            text: text[last..].to_string(),
            is_code: false,
        });
    }
    parts
}

/// Apply caveman-style compression to text, preserving code blocks.
/// Mirrors Go's CavemanCompress.
pub fn caveman_compress(text: &str, mode: CavemanMode) -> String {
    if mode == CavemanMode::Off || text.is_empty() {
        return text.to_string();
    }

    let parts = split_code_blocks(text);
    let mut result = String::with_capacity(text.len());
    for part in parts {
        if part.is_code {
            result.push_str(&part.text);
        } else {
            let compressed = match mode {
                CavemanMode::Lite => apply_lite(&part.text),
                CavemanMode::Full => apply_full(&part.text),
                CavemanMode::Ultra => apply_ultra(&part.text),
                CavemanMode::Off => part.text.clone(),
            };
            result.push_str(&compressed);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caveman_off_no_change() {
        let text = "This is a test sentence.";
        assert_eq!(caveman_compress(text, CavemanMode::Off), text);
    }

    #[test]
    fn test_caveman_lite_removes_filler() {
        let text = "just do this basically";
        let result = caveman_compress(text, CavemanMode::Lite);
        assert!(!result.contains("just"));
        assert!(!result.contains("basically"));
    }

    #[test]
    fn test_caveman_full_removes_articles() {
        let text = "the database has the error in the configuration";
        let result = caveman_compress(text, CavemanMode::Full);
        assert!(!result.starts_with("the "));
    }

    #[test]
    fn test_caveman_ultra_abbreviates() {
        let text = "the database configuration because";
        let result = caveman_compress(text, CavemanMode::Ultra);
        assert!(result.contains("DB") || result.contains("cfg") || result.contains("→"));
    }

    #[test]
    fn test_code_blocks_preserved() {
        let text = "the database\n```rust\nlet the_var = 1;\n```\nmore the text";
        let result = caveman_compress(text, CavemanMode::Full);
        assert!(result.contains("let the_var = 1;"), "code block should be preserved");
    }

    #[test]
    fn test_system_prompt() {
        assert!(!caveman_system_prompt(CavemanMode::Full).is_empty());
        assert!(caveman_system_prompt(CavemanMode::Off).is_empty());
    }
}
