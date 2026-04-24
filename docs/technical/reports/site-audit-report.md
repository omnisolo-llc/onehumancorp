# Docs Site Audit

## Findings

- The repository mixed active docs with archived design drafts and root-level release files.
- The previous docs entry point relied on embedded HTML wrappers instead of markdown-first structure.
- The docs tree had no navigation contract, no section landing pages, and no site build configuration.
- Local task tracking leaked into docs and runtime paths through `.agent-task` references.
- Several legacy source trees still lived outside `src/`.

## Remediation Applied

- Moved the remaining first-party source trees into `src/`.
- Moved stray design docs into `docs/archive/design/`.
- Centralized release documentation under `docs/public/`.
- Added `docs/index.md` and section index pages.
- Added g3doc-style navigation metadata: `_project.yaml`, `_book.yaml`, `_toc.yaml`.
- Added an MkDocs site configuration and a GitHub Pages workflow.
- Replaced local mission-file task tracking with a GitHub-issue policy.
- Updated runtime defaults from `.agent-task` to `.ohc/runtime`.

## Residual Cleanup

- A large portion of the legacy markdown corpus still contains inline HTML styling wrappers.
- Those pages remain readable, but they should be normalized to plain markdown over time.
- The issue migration seed is prepared in-docs because this environment cannot create GitHub issues directly.
