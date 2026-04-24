1. **Analyze release workflow:** Read `.github/workflows/release.yml` to understand the release process.
2. **Version Bump Strategy:** The task requires updating the version text files for a new release. We'll set the version to `0.4.6` and `0.4.6+1` since previous versions are `0.4.5` and `0.4.5+1`.
3. **Update Version Files:** Modify `.release/cloud_version.txt` and `.release/standalone_version.txt`.
4. **Update Changelog:** Append new release details to `CHANGELOG.md` based on recent commits for `0.4.6` (e.g. "refactor: standardize BuildBuddy configuration across CI and release workflows"). Ensure we capture scaling (Cloud) and privacy/offline (Standalone) improvements.
5. **Update Release Notes:** Append new release details to `RELEASE_NOTES.md`.
6. **Pre-commit Checks:** Run `pre_commit_instructions` tool and complete steps to ensure proper testing, verification, review, and reflection are done.
7. **Submit the PR:** Create the final submission.
