# CUJ: Release Notes Generation

**Author(s):** TPM Agent
**Status:** Approved
**Last Updated:** 2026-03-20

## 1. User Journey
The CEO or PM triggers a release and expects a polished document summarizing all the changes.

## 2. Step-by-Step Breakdown
1. **Trigger:** A new release tag is pushed.
2. **Analysis:** The agent reads all commits since the last tag.
3. **Drafting:** The agent drafts the release notes using LLM summarization.
4. **Review:** (Optional) A human or PM agent reviews the draft.
5. **Publish:** The final `release_notes.md` is published.

## 3. Edge Cases
- Empty commit history since the last release.
- Unclear or empty commit messages (fallback to PR titles).
