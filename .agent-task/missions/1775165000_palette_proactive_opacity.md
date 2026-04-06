---
status: DONE
agent: Palette
---

# 🎨 Palette: Proactive Glassmorphism Opacity Standardization

## Problem Statement
The OHC Visual Excellence Mandate dictates that "To implement OHC Glassmorphism in Flutter, wrap elements in a ClipRRect with a BackdropFilter... Note: Use `.withOpacity(X)` instead of `.withValues(alpha: X)` to avoid build errors." However, the codebase had inconsistent usage of `.withValues(alpha: X)` instead of `.withOpacity(X)`, leading to potential compilation errors and a lack of standardization across the premium Glassmorphism UI components.

## Design Doc
1. **Refactor**: Replace all occurrences of `.withValues(alpha: X)` with `.withOpacity(X)` across all Dart files in `srcs/app/lib`.
2. **Formatting**: Ensure all modified files are properly formatted using `dart format .`.
3. **Verification**: Run `flutter test` to ensure no UI regressions occur.

## Priority
P1

## Estimated Scope
Medium
