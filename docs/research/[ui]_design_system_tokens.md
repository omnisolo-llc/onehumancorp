# OHC Design System: Visual Excellence Tokens

## Overview
This document defines the core design tokens for the OneHumanCorp (OHC) platform. All implementer agents must adhere to these tokens to ensure the "Visual Excellence Mandate" is met across all mobile (375px) and desktop views.

## 🎨 Color Palette
OHC uses a sophisticated, dark-themed palette with high contrast and depth.

| Token | Value | Usage |
| :--- | :--- | :--- |
| `primary-gold` | `#D4AF37` | Primary buttons, success states, accents. |
| `bg-deep` | `#0A0A0A` | Main application background. |
| `surface-glass` | `rgba(255, 255, 255, 0.03)` | Card backgrounds, modal surfaces. |
| `border-glass` | `rgba(255, 255, 255, 0.08)` | Thin borders for glassmorphism effect. |
| `text-primary` | `#FFFFFF` | Main headings and body text. |
| `text-secondary` | `rgba(255, 255, 255, 0.6)` | Labels, subtexts, and secondary info. |

## 📐 Typography
OHC relies on clean, modern sans-serif fonts to provide a premium feel.

- **Primary Font**: `Outfit` (Headings, Stats, Hero text)
- **Secondary Font**: `Inter` (Body copy, forms, small labels)

| Token | Font Size | Weight | Line Height |
| :--- | :--- | :--- | :--- |
| `h1` | 32px | 700 (Bold) | 1.2 |
| `h2` | 24px | 600 (Semi-Bold) | 1.3 |
| `body-large` | 16px | 400 (Regular) | 1.5 |
| `body-small` | 14px | 400 (Regular) | 1.4 |
| `caption` | 12px | 500 (Medium) | 1.2 |

## ✨ Effects (The Glassmorphism Mandate)
Every surface in OHC must feel layered and high-end.

- **Blur**: `backdrop-filter: blur(20px) saturate(200%);`
- **Border Radius**: `12px` (Standard Card), `8px` (Buttons), `24px` (Large Sections).
- **Shadow**: `0 8px 32px 0 rgba(0, 0, 0, 0.37);`

## 🏃 Motion & Interaction
Motion should be subtle and purposeful.

- **Transition**: `200ms ease-in-out` for all hover/active states.
- **Micro-interactions**: Subtle scale up (1.02x) on button tap.
- **Skeleton Shimmer**: `linear-gradient(90deg, transparent, rgba(255,255,255,0.05), transparent)` at 1.5s duration.

## 📱 Mobile-First (375px) Constraints
- **Touch Targets**: Minimum `44x44px`.
- **Gutter**: `20px` standard horizontal padding.
- **Navigation**: Persistent bottom bar for thumb-friendly reachability.
