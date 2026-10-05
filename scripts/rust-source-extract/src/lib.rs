//! Select exact original Rust bytes; syntax selection is not compilation or authorization.

use sha2::{Digest, Sha256};
use syn::{ImplItem, Item, spanned::Spanned, visit::Visit};

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

// Syn can retain unsupported syntax as opaque tokens. Reject every Verbatim
// variant throughout the file, including outside the item being selected.
#[derive(Default)]
struct RejectOpaque(Option<proc_macro2::Span>);
macro_rules! reject_verbatim {
    ($visit:ident, $kind:ident) => {
        fn $visit(&mut self, node: &'ast syn::$kind) {
            if let syn::$kind::Verbatim(tokens) = node {
                self.0.get_or_insert(tokens.span());
            } else {
                syn::visit::$visit(self, node);
            }
        }
    };
}
impl<'ast> Visit<'ast> for RejectOpaque {
    reject_verbatim!(visit_item, Item);
    reject_verbatim!(visit_expr, Expr);
    reject_verbatim!(visit_impl_item, ImplItem);
    reject_verbatim!(visit_trait_item, TraitItem);
    reject_verbatim!(visit_foreign_item, ForeignItem);
    reject_verbatim!(visit_type, Type);
    reject_verbatim!(visit_pat, Pat);
    reject_verbatim!(visit_type_param_bound, TypeParamBound);
}

#[derive(Clone, Default)]
struct Scope {
    modules: Vec<String>,
    impl_type: Option<String>,
    impl_trait: Option<String>,
    enclosing: Vec<Span>,
}

struct Search<'a> {
    source: &'a str,
    selector: &'a Selector,
    // parse_file removes BOM/shebang bytes; all public ranges refer to the
    // original input, so restore that exact prefix length to every span.
    prefix_bytes: usize,
    matches: Vec<(Span, Vec<Span>)>,
}

impl Search<'_> {
    fn span(&self, node: &impl Spanned) -> Result<Span, String> {
        let range = node.span().byte_range();
        let start = range
            .start
            .checked_add(self.prefix_bytes)
            .ok_or("source span overflow")?;
        let end = range
            .end
            .checked_add(self.prefix_bytes)
            .ok_or("source span overflow")?;
        if start >= end || self.source.get(start..end).is_none() {
            return Err("parser returned an invalid original source span".into());
        }
        Ok(Span { start, end })
    }

    fn text(&self, node: &impl Spanned) -> Result<String, String> {
        let span = self.span(node)?;
        Ok(self.source[span.start..span.end].to_owned())
    }

    fn record(
        &mut self,
        kind: &str,
        name: &str,
        node: &impl Spanned,
        scope: &Scope,
        impl_trait: Option<&str>,
    ) -> Result<(), String> {
        if kind == self.selector.kind
            && name == self.selector.name
            && scope.modules == self.selector.modules
            && scope.impl_type == self.selector.impl_type
            && impl_trait == self.selector.impl_trait.as_deref()
        {
            self.matches
                .push((self.span(node)?, scope.enclosing.clone()));
        }
        Ok(())
    }

    fn visit(&mut self, items: &[Item], scope: &Scope) -> Result<(), String> {
        for item in items {
            let identity = match item {
                Item::Fn(node) => Some(("function", node.sig.ident.to_string())),
                Item::Struct(node) => Some(("struct", node.ident.to_string())),
                Item::Enum(node) => Some(("enum", node.ident.to_string())),
                Item::Type(node) => Some(("type", node.ident.to_string())),
                Item::Const(node) => Some(("const", node.ident.to_string())),
                Item::Static(node) => Some(("static", node.ident.to_string())),
                Item::Trait(node) => Some(("trait", node.ident.to_string())),
                Item::Mod(node) => Some(("mod", node.ident.to_string())),
                _ => None,
            };
            if let Some((kind, name)) = identity {
                self.record(kind, &name, item, scope, scope.impl_trait.as_deref())?;
            }
            match item {
                Item::Mod(module) => {
                    if let Some((_, nested_items)) = &module.content {
                        let mut nested = scope.clone();
                        nested.modules.push(module.ident.to_string());
                        nested.enclosing.push(self.span(item)?);
                        self.visit(nested_items, &nested)?;
                    }
                }
                Item::Impl(implementation) => {
                    let name = self.text(implementation.self_ty.as_ref())?;
                    let trait_name = implementation
                        .trait_
                        .as_ref()
                        .map(|(_, path, _)| self.text(path))
                        .transpose()?;
                    self.record("impl", &name, item, scope, trait_name.as_deref())?;
                    let mut nested = scope.clone();
                    nested.impl_type = Some(name);
                    nested.impl_trait = trait_name;
                    nested.enclosing.push(self.span(item)?);
                    for member in &implementation.items {
                        let identity = match member {
                            ImplItem::Fn(node) => Some(("function", node.sig.ident.to_string())),
                            ImplItem::Const(node) => Some(("const", node.ident.to_string())),
                            ImplItem::Type(node) => Some(("type", node.ident.to_string())),
                            _ => None,
                        };
                        if let Some((kind, name)) = identity {
                            self.record(
                                kind,
                                &name,
                                member,
                                &nested,
                                nested.impl_trait.as_deref(),
                            )?;
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

pub fn extract(source: &[u8], selector: &Selector) -> Result<Extraction, String> {
    let text =
        std::str::from_utf8(source).map_err(|error| format!("source is not UTF-8: {error}"))?;
    if !matches!(
        selector.kind.as_str(),
        "function" | "struct" | "enum" | "type" | "const" | "static" | "trait" | "mod" | "impl"
    ) {
        return Err(format!("unsupported Rust item kind: {}", selector.kind));
    }
    if selector.impl_trait.is_some() && selector.impl_type.is_none() && selector.kind != "impl" {
        return Err("unsupported trait method selector without impl type".into());
    }
    let tree = syn::parse_file(text).map_err(|error| {
        let position = error.span().start();
        format!(
            "invalid Rust syntax at line {}, column {}: {error}",
            position.line,
            position.column + 1
        )
    })?;
    let mut guard = RejectOpaque::default();
    guard.visit_file(&tree);
    if let Some(span) = guard.0 {
        return Err(format!(
            "unsupported opaque Rust syntax at line {}, column {}",
            span.start().line,
            span.start().column + 1
        ));
    }
    let prefix_bytes = if text.starts_with('\u{feff}') {
        '\u{feff}'.len_utf8()
    } else {
        0
    } + tree.shebang.as_ref().map_or(0, String::len);
    let mut search = Search {
        source: text,
        selector,
        prefix_bytes,
        matches: Vec::new(),
    };
    search.visit(&tree.items, &Scope::default())?;
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
