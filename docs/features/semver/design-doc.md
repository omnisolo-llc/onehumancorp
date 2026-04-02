<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Design Doc: Semantic Versioning Automation

**Author(s):** TPM Agent
**Status:** Approved
**Last Updated:** 2026-03-20

## 1. Overview
Automates the calculation of the next Semantic Version (SemVer) based on the commit history and PR labels.

## 2. Architecture
- **Analysis:** Analyzes commit messages (e.g., "feat:", "fix:", "BREAKING CHANGE") to determine the bump type (major, minor, patch).
- **Execution:** Reads the current latest git tag and calculates the next tag.

## 3. Implementation Details
- Uses Go and standard git parsing libraries or direct git command execution in isolated runners.


</div>
