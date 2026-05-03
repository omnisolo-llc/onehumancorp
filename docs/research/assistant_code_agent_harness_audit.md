# Assistant Code Agent Harness Audit

The agent harness needs to validate commands before they are executed to prevent bypassing sandbox restrictions via shell tricks.
The following operations need to be prevented:
- Zsh module loading (`zmodload`, `emulate`)
- Subshell obfuscation (`<()`, `>()`, `=cmd`)
- Attempted access to OHC internal `sip.db` state files
