<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

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


</div>
