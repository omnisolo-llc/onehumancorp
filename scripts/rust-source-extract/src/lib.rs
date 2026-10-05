//! Select exact original Rust bytes; syntax selection is not compilation or authorization.

use sha2::{Digest, Sha256};
use tree_sitter::{Node, Parser};

#[derive(Default)]
pub struct Selector {
    pub kind: String,
    pub name: String,
    pub modules: Vec<String>,
    pub impl_type: Option<String>,
    pub impl_trait: Option<String>,
}

impl Selector {
    pub fn new(kind: &str, name: &str) -> Self {
        Self {
            kind: kind.into(),
            name: name.into(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct Extraction {
    pub start: usize,
    pub end: usize,
    pub source_sha256: String,
    pub selection_sha256: String,
    pub enclosing: Vec<Span>,
}

pub fn sha256(source: &[u8]) -> String {
    format!("{:x}", Sha256::digest(source))
}

fn field_text<'a>(node: Node<'_>, field: &str, source: &'a [u8]) -> Option<&'a str> {
    node.child_by_field_name(field)
        .map(|value| std::str::from_utf8(&source[value.byte_range()]).expect("validated UTF-8"))
}

fn item_span(node: Node<'_>, source: &[u8]) -> Span {
    let mut start = node.start_byte();
    let mut previous = node.prev_named_sibling();
    while let Some(sibling) = previous {
        match sibling.kind() {
            "attribute_item" => start = sibling.start_byte(),
            "line_comment" | "block_comment" => {
                let text = &source[sibling.byte_range()];
                let outer_doc = (text.starts_with(b"///") && !text.starts_with(b"////"))
                    || (text.starts_with(b"/**") && !text.starts_with(b"/***"));
                if outer_doc {
                    start = sibling.start_byte();
                } else if text.starts_with(b"//!") || text.starts_with(b"/*!") {
                    break;
                }
            }
            _ => break,
        }
        previous = sibling.prev_named_sibling();
    }
    Span {
        start,
        end: node.end_byte(),
    }
}

fn reject_invalid_syntax(node: Node<'_>) -> Result<(), String> {
    if node.is_error() || node.is_missing() {
        let position = node.start_position();
        return Err(format!(
            "invalid Rust syntax ({}{}) at line {}, column {}",
            node.kind(),
            if node.is_missing() { ", missing" } else { "" },
            position.row + 1,
            position.column + 1
        ));
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        reject_invalid_syntax(child)?;
    }
    Ok(())
}

#[derive(Clone, Default)]
struct Scope {
    modules: Vec<String>,
    impl_type: Option<String>,
    impl_trait: Option<String>,
    enclosing: Vec<Span>,
}

struct Search<'a> {
    source: &'a [u8],
    selector: &'a Selector,
    node_kind: &'a str,
    matches: Vec<(Span, Vec<Span>)>,
}

impl Search<'_> {
    fn visit(&mut self, container: Node<'_>, scope: &Scope) {
        let mut cursor = container.walk();
        for node in container.named_children(&mut cursor) {
            let is_impl = node.kind() == "impl_item";
            let name = field_text(node, if is_impl { "type" } else { "name" }, self.source);
            let selected_trait = if is_impl {
                field_text(node, "trait", self.source)
            } else {
                scope.impl_trait.as_deref()
            };
            if node.kind() == self.node_kind
                && name == Some(self.selector.name.as_str())
                && scope.modules == self.selector.modules
                && scope.impl_type == self.selector.impl_type
                && selected_trait == self.selector.impl_trait.as_deref()
            {
                self.matches
                    .push((item_span(node, self.source), scope.enclosing.clone()));
            }
            if (node.kind() == "mod_item" || is_impl)
                && let Some(body) = node.child_by_field_name("body")
            {
                let mut nested = scope.clone();
                nested.enclosing.push(item_span(node, self.source));
                if is_impl {
                    nested.impl_type = name.map(str::to_owned);
                    nested.impl_trait = field_text(node, "trait", self.source).map(str::to_owned);
                } else if let Some(name) = name {
                    nested.modules.push(name.to_owned());
                }
                self.visit(body, &nested);
            }
        }
    }
}

pub fn extract(source: &[u8], selector: &Selector) -> Result<Extraction, String> {
    std::str::from_utf8(source).map_err(|error| format!("source is not UTF-8: {error}"))?;
    let node_kind = match selector.kind.as_str() {
        "function" => "function_item",
        "struct" => "struct_item",
        "enum" => "enum_item",
        "type" => "type_item",
        "const" => "const_item",
        "static" => "static_item",
        "trait" => "trait_item",
        "mod" => "mod_item",
        "impl" => "impl_item",
        other => return Err(format!("unsupported Rust item kind: {other}")),
    };
    if selector.impl_trait.is_some() && selector.impl_type.is_none() && selector.kind != "impl" {
        return Err("unsupported trait method selector without impl type".into());
    }
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .map_err(|error| error.to_string())?;
    let tree = parser
        .parse(source, None)
        .ok_or("Rust parser returned no tree")?;
    reject_invalid_syntax(tree.root_node())?;
    let mut search = Search {
        source,
        selector,
        node_kind,
        matches: Vec::new(),
    };
    search.visit(tree.root_node(), &Scope::default());
    match search.matches.len() {
        0 => Err(format!(
            "Rust {} {:?} not found in selected module/impl",
            selector.kind, selector.name
        )),
        1 => {
            let (span, enclosing) = search.matches.pop().expect("one match");
            Ok(Extraction {
                source_sha256: sha256(source),
                selection_sha256: sha256(&source[span.start..span.end]),
                start: span.start,
                end: span.end,
                enclosing,
            })
        }
        count => Err(format!(
            "ambiguous Rust {} {:?}: {count} matches",
            selector.kind, selector.name
        )),
    }
}
