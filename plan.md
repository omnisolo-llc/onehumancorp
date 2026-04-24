1. **Version Bump:** Calculate version bumps for both Cloud server pods and Standalone desktop binaries. Update `.release/cloud_version.txt` and `.release/standalone_version.txt` to `v0.4.6` and `v0.4.6+1` respectively.
2. **Changelog Synthesis:** Update `CHANGELOG.md` and `RELEASE_NOTES.md` adding new release notes highlighting both scaling (Cloud) and privacy/offline (Standalone) improvements.
3. **Pre-commit Steps:** Run `pre_commit_instructions` to ensure proper testing, verification, review, and reflection are done.
4. **Trigger Github workflow:** Submit PR.
