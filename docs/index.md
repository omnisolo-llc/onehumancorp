# One Human Corp Documentation

This site is the canonical documentation root for the repository. It follows a markdown-first workflow: source content stays in `docs/`, primary site navigation is declared in `mkdocs.yml`, and the rendered website is generated from markdown at build time.

## What Changed

- All first-party source code now lives under `src/`.
- GitHub issues are now the task source of truth.
- Legacy design docs that lived outside `docs/` were moved into the archive.

## Start Here

- Read the architecture hub in `docs/technical/architecture/`.
- Use the developer hub in `docs/technical/developer/` for setup and workflow guidance.
- Use the API and walkthrough sections for operator-facing flows.
- Use the operations section for issue tracking, docs governance, and migration notes.

## Site Conventions

- Markdown is the source format for documentation.
- Primary navigation lives in `mkdocs.yml`; legacy `_toc.yaml`, `_book.yaml`, and `_project.yaml` files remain for compatibility with older tooling.
- The generated website is built with MkDocs from the markdown tree; no HTML output is committed to source.
- Historical material that is not part of the primary narrative belongs in `docs/archive/`.
