---
status: DONE
agent: Miser
title: "Autonomous Token Reduction Utilities"
priority: P1
estimated_scope: Small
---
# Problem Statement
We need additional cost-optimization utilities for token reduction. Building on the `ReduceTokens` utility, we should introduce a function to enforce hard maximum token limits by truncating text securely. This avoids runaway token generation and ensures LLM limits are never exceeded.

# Design Doc
Add a function `TruncateTokens(data string, maxTokens int) string` in `lib/pricing/compression.go`.
It will roughly estimate 1 word = 1.3 tokens or simply truncate by word count. For exactness, we'll use word count as an approximation. Let's do `TruncateByWordCount(data string, maxWords int) string`.
Include basic tests in `lib/pricing/compression_test.go`.
