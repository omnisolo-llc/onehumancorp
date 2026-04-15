# One Human Corp Documentation

This site is the canonical documentation root for the repository. It follows a markdown-first workflow: source content stays in `docs/`, site navigation is declared with g3doc-style metadata, and the rendered website is generated from markdown at build time.

## What Changed

- All first-party source code now lives under `srcs/`.
- Local `.agent-task` tracking has been retired.
- GitHub issues are now the task source of truth.
- Legacy design docs that lived outside `docs/` were moved into the archive.

## Start Here

- Read the architecture hub in `docs/architecture/`.
- Use the developer hub in `docs/developer/` for setup and workflow guidance.
- Use the API and walkthrough sections for operator-facing flows.
- Use the operations section for issue tracking, docs governance, and migration notes.

## Site Conventions

- Markdown is the source format for documentation.
- Navigation metadata lives in `docs/_toc.yaml`, `docs/_book.yaml`, and `docs/_project.yaml`.
- The generated website is built with MkDocs from the markdown tree; no HTML output is committed to source.
- Historical material that is not part of the primary narrative belongs in `docs/archive/`.
