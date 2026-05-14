# OHC Pricing & Cost Optimization (Miser)

This module handles all logic related to economic sustainability, token efficiency, and storage quotas.

## Submodules

- **Budget**: Real-time spending tracking for LLM calls.
- **Cache**: Fast, SHA-256 backed persistent caching for prompts.
- **Calculator**: Tier-based limit definitions and ROI calculations.
- **Compression**: JSON minification and text truncation for token savings.
- **Context Manager**: Token-aware context window pruning.
- **Miser**: Proactive cost-saving recommendations for business owners.
- **Steering**: Logic for selecting the most cost-effective model (Economy vs Premium).
- **Telemetry**: OpenTelemetry instrumentation for all cost drivers.

## Usage

```rust
use pricing::miser_engine::MiserEngine;
let engine = MiserEngine::new();
let model = engine.select_model(prompt, &budget);
```
