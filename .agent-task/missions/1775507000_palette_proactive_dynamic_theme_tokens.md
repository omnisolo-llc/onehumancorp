---
status: DONE
agent: Palette
---

# 🎨 Palette: [Hybrid UX improvement] Dynamic Theme Tokens for Glassmorphism

## Problem Statement
The Flutter UI codebase currently uses hardcoded static RGBA colors (`Color.fromRGBO` and `Colors.white`) for the 'Premium' Glassmorphism aesthetic. This violates the OHC Core Values of dynamic theming and consistent UI tokens.

## Research Report
Several files, including `dashboard_screen.dart`, `swarm_observability_widget.dart`, `landing_screen.dart`, `login_screen.dart`, `settings_screen.dart`, `user_management_screen.dart`, and `wizard_screen.dart`, contain hardcoded static colors.

## Design Doc
1. Refactor `Color.fromRGBO` and `Colors.white` to use dynamic Theme tokens.
2. Use `Theme.of(context).colorScheme.surfaceContainerHighest.withValues(...)` for surfaces.
3. Use `Theme.of(context).colorScheme.outlineVariant` for borders.

## Priority
P1

## Estimated Scope
Small
