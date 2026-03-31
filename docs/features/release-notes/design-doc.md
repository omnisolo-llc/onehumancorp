# Design Doc: Release Notes Generation

**Author(s):** TPM Agent
**Status:** Approved
**Last Updated:** 2026-03-20

## 1. Overview
The Release Notes Generation feature enables an AI agent to automatically generate comprehensive and user-friendly release notes based on the project's commit history, PR descriptions, and issue trackers.

## 2. Architecture
- **Data Gathering:** The agent interacts with the VCS (e.g., Git) and project management tools via MCP to fetch commits and closed issues.
- **Processing:** Uses LLMs to categorize changes (Features, Bug Fixes, Chores) and summarize them in a human-readable format.
- **Output:** Generates a Markdown file (`release_notes.md`) which can be published to GitHub Releases, documentation sites, or sent via email.

## 3. Developer Workflow
Triggered automatically during the CI/CD pipeline when a new tag is pushed or manually via the Hub.

## 4. Implementation Details
- Stack: Go for orchestration, LLM via MCP for text generation.
- Data Mocks: Must use real commit history during testing.
