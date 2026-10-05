//! Bounded HTML5 parsing for untrusted web responses.
//!
//! The upstream tokenizer and tree builder own HTML syntax and repair. This
//! adapter only bounds their work and discards the complete result on rejection.
use dom_query::{Document, NodeId};
use html5ever::TokenizerResult;
use html5ever::tokenizer::{BufferQueue, TagKind, Token, TokenSink, TokenSinkResult, Tokenizer};
use html5ever::tree_builder::{Tracer, TreeBuilder, TreeBuilderOpts, TreeSink};
use std::cell::Cell;

const CHUNK_BYTES: usize = 512;
const MAX_UNFINISHED_BYTES: usize = 4096;
const MAX_TOKENS: usize = 32_768;
const MAX_START_TAGS: usize = 4096;
const MAX_ATTRIBUTES: usize = 64;
const MAX_ACTIVE_HANDLES: usize = 128;
const MAX_TREE_WORK: usize = 131_072;
const MAX_CHARACTER_WORK: usize = 16 * 1024 * 1024;

struct HandleCounter<'a> {
    document: &'a Document,
    handles: Cell<usize>,
    attributes: Cell<usize>,
}

impl Tracer for HandleCounter<'_> {
    type Handle = NodeId;

    fn trace_handle(&self, node: &NodeId) {
        // Count duplicates too: both open elements and active formatting entries
        // contribute to tree-builder work, even when they reference one node.
        self.handles.set(self.handles.get() + 1);
        let attributes = self.document.tree.query_node_or(node, 0, |node| {
            node.as_element().map_or(0, |element| element.attrs.len())
        });
        self.attributes.set(self.attributes.get() + attributes);
    }
}

struct BudgetSink {
    tree: TreeBuilder<NodeId, Document>,
    failure: Cell<Option<&'static str>>,
    unfinished_bytes: Cell<usize>,
    tokens: Cell<usize>,
    starts: Cell<usize>,
    tree_work: Cell<usize>,
    character_work: Cell<usize>,
}

impl BudgetSink {
    fn new() -> Self {
        Self {
            tree: TreeBuilder::new(
                Document::default(),
                TreeBuilderOpts {
                    // Match Document::from, including noscript parsing behavior.
                    scripting_enabled: false,
                    ..Default::default()
                },
            ),
            failure: Cell::new(None),
            unfinished_bytes: Cell::new(0),
            tokens: Cell::new(0),
            starts: Cell::new(0),
            tree_work: Cell::new(0),
            character_work: Cell::new(0),
        }
    }

    fn active_work(&self) -> (usize, usize) {
        let counter = HandleCounter {
            document: &self.tree.sink,
            handles: Cell::new(0),
            attributes: Cell::new(0),
        };
        self.tree.trace_handles(&counter);
        (counter.handles.get(), counter.attributes.get())
    }

    fn reject(&self, reason: &'static str) -> TokenSinkResult<NodeId> {
        self.failure.set(Some(reason));
        // Non-tag tokens require Continue in html5ever. Skip all further tree
        // work; the driver rejects as soon as this bounded feed returns.
        TokenSinkResult::Continue
    }
}

impl TokenSink for BudgetSink {
    type Handle = NodeId;

    fn process_token(&self, token: Token, line_number: u64) -> TokenSinkResult<NodeId> {
        if self.failure.get().is_some() {
            return TokenSinkResult::Continue;
        }
        self.tokens.set(self.tokens.get() + 1);
        if self.tokens.get() > MAX_TOKENS {
            return self.reject("token budget");
        }
        if let Token::TagToken(tag) = &token {
            if tag.attrs.len() > MAX_ATTRIBUTES {
                return self.reject("attribute budget");
            }
            if tag.kind == TagKind::StartTag {
                self.starts.set(self.starts.get() + 1);
                if self.starts.get() > MAX_START_TAGS {
                    return self.reject("start-tag budget");
                }
            }
        }

        let (active, attributes) = self.active_work();
        if active > MAX_ACTIVE_HANDLES {
            return self.reject("active-handle budget");
        }
        // Formatting reconstruction clones attribute vectors, and repeated body
        // or html tags can merge attributes into an existing node. Charge actual
        // live DOM attributes before every token, including deferred EOF work.
        self.tree_work
            .set(self.tree_work.get() + active + attributes);
        if self.tree_work.get() > MAX_TREE_WORK {
            return self.reject("tree-work budget");
        }
        if let Token::CharacterTokens(text) = &token {
            // The tree builder can split character tokens and defer table text.
            // Count bytes before forwarding, not only visible token callbacks.
            self.character_work
                .set(self.character_work.get() + text.len() * active);
            if self.character_work.get() > MAX_CHARACTER_WORK {
                return self.reject("character-work budget");
            }
        }

        let completed = match &token {
            Token::ParseError(_) | Token::EOFToken => false,
            Token::CharacterTokens(text) => !text.is_empty(),
            _ => true,
        };
        if completed {
            self.unfinished_bytes.set(0);
        }
        let result = self.tree.process_token(token, line_number);
        // A token can add implicit/reconstructed elements. Inspect actual parser
        // handles after it, instead of guessing depth from start/end tag syntax.
        if self.active_work().0 > MAX_ACTIVE_HANDLES {
            self.failure.set(Some("active-handle budget"));
        }
        result
    }

    fn end(&self) {
        if self.failure.get().is_none() {
            self.tree.end();
        }
    }

    fn adjusted_current_node_present_but_not_in_html_namespace(&self) -> bool {
        self.tree
            .adjusted_current_node_present_but_not_in_html_namespace()
    }
}

pub(super) fn parse_html(html: &str) -> Result<Document, &'static str> {
    parse_html_chunked(html, CHUNK_BYTES)
}

fn parse_html_chunked(html: &str, chunk_bytes: usize) -> Result<Document, &'static str> {
    assert!((1..=CHUNK_BYTES).contains(&chunk_bytes));
    let tokenizer = Tokenizer::new(BudgetSink::new(), Default::default());
    let input = BufferQueue::default();
    let mut start = 0;
    while start < html.len() {
        let mut end = (start + chunk_bytes).min(html.len());
        while !html.is_char_boundary(end) {
            end -= 1;
        }
        if end == start {
            end += html[start..].chars().next().unwrap().len_utf8();
        }
        // Tokenizer fields such as comments, attribute names/values and character
        // references can grow before any completed token reaches our sink. Do
        // not let parse errors reset this budget. The uncounted suffix following
        // the last completed token is at most one 512-byte chunk, so the buffered
        // unfinished token is bounded by 4096 + 512 bytes in production.
        let pending = tokenizer.sink.unfinished_bytes.get() + end - start;
        if pending > MAX_UNFINISHED_BYTES {
            return Err("unfinished-token budget");
        }
        tokenizer.sink.unfinished_bytes.set(pending);
        input.push_back(html[start..end].into());
        loop {
            let result = tokenizer.feed(&input);
            if let Some(reason) = tokenizer.sink.failure.get() {
                return Err(reason);
            }
            if matches!(result, TokenizerResult::Done) {
                break;
            }
            // Match upstream's driver: resume Script/EncodingIndicator results.
            // No script is executed and decoding was already done by the caller.
        }
        start = end;
    }
    tokenizer.end();
    if let Some(reason) = tokenizer.sink.failure.get() {
        return Err(reason);
    }
    Ok(tokenizer.sink.tree.sink.finish())
}

#[cfg(test)]
#[path = "html_parser_test.rs"]
mod tests;
