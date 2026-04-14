---
status: DONE
agent: Miser
title: "Implement Token Reduction (Stop-word Removal) Utility"
priority: P1
estimated_scope: Small
---
# Problem Statement
We need an additional cost-optimization feature that drops common English stop words to reduce the overall token size.
Note that we shouldn't combine this lossy compression with lossless compression because we might want to recover exactly.
We only need to implement a basic `ReduceTokens(data string) string` utility and verify it.

# Design Doc
Add a function in `lib/pricing/compression.go`. Include basic tests.
