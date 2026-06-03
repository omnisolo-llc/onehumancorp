# OHC Premium Design System Tokens

Standard design tokens for OneHumanCorp to ensure consistency across AI-generated and hand-coded interfaces.

## Colors
| Token | Value | Description |
|---|---|---|
| `glass-bg` | `rgba(255, 255, 255, 0.65)` | Standard light mode glass background |
| `glass-bg-dark` | `rgba(22, 22, 26, 0.7)` | Standard dark mode glass background |
| `glass-border` | `1px solid rgba(255, 255, 255, 0.4)` | Glass container border |
| `text-primary` | `#ffffff` | Primary text on glass |

## Effects
| Token | Value | Description |
|---|---|---|
| `glass-blur` | `blur(30px) saturate(210%)` | Standard macOS-style glass blur |
| `card-shadow` | `0 8px 32px 0 rgba(0, 0, 0, 0.3)` | Deep shadow for glass panels |

## Motion
| Token | Value | Description |
|---|---|---|
| `fade-in` | `fade-in 0.4s ease-out` | Default entrance animation |
| `shake` | `shake 0.2s ease-in-out` | Error/Attention feedback |

## Typography
- **Primary**: 'Outfit'
- **Secondary**: 'Inter'
- **Fallback**: sans-serif

## CSS Classes (Tailwind-compatible)
- `.ohc-hybrid-panel`: Full modular dashboard card styling.
- `.mac-glass-container`: Base glass material.
- `.animate-fade-in`: Standard entrance.
