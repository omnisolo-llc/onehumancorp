# Design Brief: OHC Premium Design System

## Title
The OHC Visual Excellence Mandate: Tokens & Standards

## Problem Statement
Small business owners associate "cheap" looking websites with poor service. To empower non-technical users to compete with large brands, OHC must provide a world-class, premium visual experience out of the box. Every interaction must feel expensive, reliable, and modern.

## Design Tokens

### Typography
- **Primary (Headings)**: `Outfit` — A modern, geometric sans-serif that conveys confidence and friendliness.
- **Secondary (Body)**: `Inter` — Highly legible, neutral, and professional. Optimized for mobile reading.

### Visual Style: Glassmorphism
OHC uses a "Glass" metaphor to create depth and hierarchy without clutter.
- **Background Blur**: `20px` (or `15px` for smaller elements).
- **Surface Color**: `rgba(255, 255, 255, 0.03)` (Light) / `rgba(0, 0, 0, 0.2)` (Dark).
- **Border**: `1px solid rgba(255, 255, 255, 0.1)`.
- **Saturation**: `200%` (on background elements to make colors "pop" through the glass).

### Motion & Animation
Animations must feel "organic" and purposeful.
- **Entrance**: `300ms`, `cubic-bezier(0.4, 0, 0.2, 1)`.
- **Exit**: `200ms`, `ease-in`.
- **Feedback**: Subtle haptic-style shimmers for loading and "success" states.

### Mobile-First Layout (375px Baseline)
- **Safe Zones**: Strictly adhere to iOS/Android safe areas.
- **Touch Targets**: Minimum `44x44px` for all interactive elements.
- **Gutter**: `16px` standard horizontal padding.
- **Rounded Corners**: `12px` (Small), `24px` (Large cards), `50%` (Pills/Buttons).

## The "Grandmother Test" (UX Principles)
1.  **Zero Jargon**: Never use "DNS," "API," or "SKU." Use "Website Address," "Connection," and "Item."
2.  **1-Tap Actions**: Complex tasks must be boiled down to a single button tap whenever possible.
3.  **Proactive Context**: If the app asks for information, it must explain *why* in one simple sentence.

## Implementation Prompt
**To Implementer Agent:**
Implement the OHC Design System tokens in the Slint/Flutter frontend. Create a reusable `GlassCard` component that applies the standard blur, border, and background tokens. Update the global theme to use `Outfit` and `Inter`. Ensure all button components meet the `44x44px` touch target requirement. Implement the "Skeleton Shimmer" loading state for all data-fetching components.

## Priority
P0 (Brand Identity)

## Estimated Scope
Medium
