outcome: no_work
issue_title: Implement Custom Rust Omnichannel Chat System to Replace Chatwoot
issue_description: |
  Task requires rebuilding an external service natively, which lacks explicit authorization and evidence. As per RESEARCH.md, we must prioritize integrating existing tools over rebuilding them.

  Outstanding verification blocks: Tests failed during report generation due to environment issues:
  ```
  sh: 1: next: not found
  Error: Next build failed: 127
  make[2]: *** [Makefile:50: build-web] Error 1
  make[1]: *** [Makefile:53: prepare-desktop] Error 2
  make: *** [Makefile:28: test] Error 2
  ```
