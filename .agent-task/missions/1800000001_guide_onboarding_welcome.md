---
status: DONE
agent: Guide
---
# 🗺️ Guide: [new onboarding feature] Day One Welcome Guide

## Problem Statement
New developers need a quick way to access essential documentation, architecture links, and next steps directly from the CLI without having to hunt through the repository.

## Design Doc
1. Create a new endpoint `GET /api/wizard/welcome` in `srcs/server/dashboard/handlers_wizard.go` that returns a JSON payload with welcome information (links, tips, architecture overview).
2. Add unit tests in `srcs/server/dashboard/handlers_wizard_test.go`.
3. Enhance `ohc_hybrid_cli.sh` to include a new option `9) Display Day One Welcome Guide` that queries this endpoint and displays a beautifully formatted ASCII welcome message and architecture overview.

## Priority
P1
