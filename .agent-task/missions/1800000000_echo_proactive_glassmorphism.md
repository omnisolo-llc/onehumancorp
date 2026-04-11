---
status: DONE
agent: Implementer
---

# Title: Proactive UI Friction Fix: Unified Glassmorphism Rendering

## Problem Statement
The OHC Visual Excellence Mandate requires UI components to feature Glassmorphism with a 20px blur and high-saturation colors. Several screens in the Flutter application (`landing_screen.dart`, `agent_hire_wizard_screen.dart`, etc.) currently implement a simple `ImageFilter.blur` without the accompanying `ColorFilter.matrix` for high-saturation, resulting in a UI that feels "cheap" and inconsistent with the premium OHC layout tokens. Additionally, failing to use `ColorFilter.matrix` inside `ImageFilter.compose` can lead to `ArgumentError` exceptions during widget tests.

## Research Report
A proactive visual audit of the Flutter codebase revealed that multiple screens deviate from the premium design standards set forth in `CLAUDE_OHC.md` and the team's visual intent logs.
- Affected components: BackdropFilters lacking proper composition.
- Fix approach: Wrap existing `ImageFilter.blur` in `ImageFilter.compose` and apply the standard 5x4 saturation matrix on the `outer` parameter.

## Design Doc
N/A - Direct visual implementation fix matching existing tokens.

## Implementation Prompt
Replace instances of simple `ImageFilter.blur` with `ImageFilter.compose` applying the OHC standard saturation matrix in all relevant Dart screen files.

## Priority
P1

## Estimated Scope
Small
