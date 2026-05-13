# [strategy] Market Feature Gap Matrix

## Matrix Overview

| Feature | Shopify | Wix | Squarespace | OHC (Current Gap) | OHC (Target Advantage) |
|---------|---------|-----|-------------|-------------------|------------------------|
| **Setup Speed** | Hours/Days | Hours | Days | None | **< 10 Minutes via AI** |
| **Mobile-First App** | Poor (Setup) | Average | Average | Missing | **100% Native Mobile UX** |
| **Auto-Replies (AI)**| Needs 3rd Party App | No | No | Missing | **Built-in Core Feature** |
| **Inventory Sync** | Excellent | Good | Good | Basic | **Edge-synced Real-time** |
| **Social Media Gen** | Needs App | Basic | No | Missing | **1-Tap Proactive Agent** |
| **Booking/Calendar** | Needs App | Add-on | Add-on | Missing | **Native AI Scheduling** |
| **Plain English Data**| No | No | No | Missing | **Daily Narrative Push** |

## Identified Gaps for OHC Codebase
1. **Mobile Onboarding Flow**: Current OHC implementation assumes desktop usage. Needs a 375px responsive wizard.
2. **AI Auto-Reply Module**: Backend lacks the webhook integrations for Instagram/WhatsApp DMs to feed into an LLM.
3. **Proactive Agent Feed**: We need a UI component (Activity Feed) where agents propose actions (e.g., "Post to Instagram?") for 1-tap approval.

```mermaid
radarChart
    title Feature Superiority Radar
    axes Mobile UX, AI Automation, Ease of Setup, E-commerce Depth, Affordability
    "Shopify" : 3, 2, 2, 5, 2
    "Wix" : 3, 3, 4, 3, 4
    "OHC (Target)" : 5, 5, 5, 4, 5
```
