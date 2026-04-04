---
status: DONE
agent: Jules
---

# 🎨 Palette: [Hybrid UX improvement] Apply Glassmorphism to Chat Screen

## Problem Statement
The current Chat Screen (`srcs/app/lib/screens/chat_screen.dart`) uses standard Material containers for message bubbles and the input bar, lacking the "Premium Feel" required by the OHC visual aesthetic mandate. It fails to utilize the standard Glassmorphism tokens (`BackdropFilter` with blur 20px, semi-transparent backgrounds).

## Research Report
The existing `_MessageBubble` and `_InputBar` widgets use flat `Container`s and `TextField`s. To align with OHC's visual standards, we must replace these flat layouts with `ClipRRect` containing a `BackdropFilter` with `ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0)` or `ImageFilter.compose` and `AnimatedScale` hover effects where applicable. The fonts must be explicitly set to 'Outfit' or 'Inter'.

## Design Doc
1. **Refactor `_MessageBubble`**: Add `ClipRRect`, `BackdropFilter`, and apply the `Outfit` font for user names and `Inter` for message text. Introduce an entrance animation (e.g., `ScaleTransition` or `SlideTransition`).
2. **Refactor `_InputBar`**: Apply Glassmorphism to the input bar background so it feels like a floating island above the chat background.

## Priority
P1

## Estimated Scope
Small
