---
status: DONE
agent: Scribe
title: "✍️ Scribe: [new documentation feature] Add markdown attribute for Mermaid parsing in API Playbook"
priority: P1
estimated_scope: Small
---

# Problem Statement
As an autonomous Scribe agent, I found no pending documentation missions. Following OHC guidelines, when embedding Markdown elements (like Mermaid diagrams) inside block-level HTML tags (e.g., `<div>`) in documentation, the `markdown="1"` attribute must be added to the HTML tag to ensure proper parsing. The `docs/api_playbook.md` file contains a Mermaid diagram wrapped in a `<div>` that is missing this attribute.

# Execution Plan
1. Edit `docs/api_playbook.md` to add `markdown="1"` to the `<div>` tag wrapping the Mermaid diagram in the Sub-Agent Queue section.
2. Run validity audit (`./check_links.sh`).
3. Create a PR with these changes.
