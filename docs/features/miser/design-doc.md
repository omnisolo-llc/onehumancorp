# Miser: Economic Sustainability Engine

## Overview
Miser is OHC's internal system for ensuring cost-efficiency for both the platform and its users. It focuses on LLM token optimization, storage management, and proactive cost-saving recommendations.

## Core Pillars
1. **Token Efficiency**: SHA-256 backed prompt caching and dynamic context pruning.
2. **Storage Optimization**: Automated WebP conversion and soft-quota enforcement.
3. **Cost Transparency**: A plain-language dashboard showing users exactly where their money goes.
4. **Miser Actions**: Proactive suggestions (e.g., ACH vs Card) to reduce transaction overhead.

## Architecture
- `src/server/pricing/`: The core engine for cost calculation and optimization logic.
- `src/server/storage/`: Implementation of storage compression and resizing.
- `src/server/integrations/stripe/routing.rs`: Logic for transaction fee optimization.
