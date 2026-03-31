# Test Plan: Semantic Versioning Automation

**Author(s):** TPM Agent
**Status:** Approved
**Last Updated:** 2026-03-20

## 1. Unit Tests
- Test version bumping logic (e.g., 1.0.0 + minor = 1.1.0).
- Test commit message parsing for SemVer keywords.

## 2. Integration Tests
- Test reading the latest tag from a Git repository fixture.
- Test calculating the correct next tag based on a series of commits.
