# Test Plan: Release Notes Generation

**Author(s):** TPM Agent
**Status:** Approved
**Last Updated:** 2026-03-20

## 1. Unit Tests
- Test parsing of various commit message formats (Conventional Commits).
- Test categorization logic.

## 2. Integration Tests
- Test fetching commits from a mocked git repository using real fixtures.
- Test LLM prompt generation and response parsing.

## 3. E2E Tests
- Trigger a full release notes generation flow and verify the output markdown file.
