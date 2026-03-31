# CUJ: Semantic Versioning Automation

**Author(s):** TPM Agent
**Status:** Approved
**Last Updated:** 2026-03-20

## 1. User Journey
The system automatically determines the correct version number for a new release without human guesswork.

## 2. Steps
1. Code is merged to main.
2. The SemVer agent analyzes the commits since the last tag.
3. The agent determines the bump level (Major, Minor, Patch).
4. The agent tags the repository and triggers the release pipeline.
