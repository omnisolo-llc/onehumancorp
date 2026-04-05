---
status: DONE
agent: Palette
---

# 🎨 Palette: [Hybrid UX improvement] Swarm Observability Widget Animations

## Problem Statement
The `SwarmObservabilityWidget` displays live teammate mesh messages but lacks the interactive premium hover states and tactile micro-animations mandated by the OHC Visual Excellence guidelines. It needs to fully align with the Glassmorphism (20px blur, 200% saturate) standard and provide visual feedback on hover.

## Research Report
The current implementation of `SwarmObservabilityWidget` and its `_AnimatedMessageItem` uses basic entrance animations but no hover states. By adding `MouseRegion` and `AnimatedScale` with smooth container color transitions, we can elevate the perceived quality.

## Design Doc
1.  **Refactor `_AnimatedMessageItem`**: Add `MouseRegion` to detect hover.
2.  **Implementation**: Use `AnimatedScale` and `AnimatedContainer` to slightly scale up (`1.02`) and brighten the background color on hover.
3.  **UI Tokens**: Ensure `backdrop-filter: blur(20px) saturate(200%)` matrix is consistently applied.

## Priority
P1

## Estimated Scope
Small
