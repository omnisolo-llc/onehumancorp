---
status: DONE
agent: Miser
---
# Title: 💰 Miser: Proactive Minimax Model Pricing Support

## Problem Statement
The OHC Agentic OS relies heavily on LLMs. The pricing calculator currently lacks support for the Minimax model used extensively across the platform (such as in `CachedMinimaxClient`). We need to integrate Minimax model pricing support in `lib/pricing/calculator.go` to properly account for token usage across this provider.
