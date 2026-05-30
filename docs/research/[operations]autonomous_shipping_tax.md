# [operations] Autonomous Shipping & Tax Configuration Agent

**Problem Statement**: Setting up shipping zones and tax rates is the #1 reason non-technical users abandon store setup. Maya (Baker) shouldn't need to understand tax jurisdictions to sell a cake.

**Research Report**: Shopify requires manual zone setup. Wix relies on third-party integrations (Avalara). SMBs want this abstracted entirely.

**Design Doc**:
- *Trigger*: User defines business type (e.g., "Local pickup only" or "Shipping nationwide").
- *AI Agent (Finance & Operations)*: Automatically sets local tax rates based on the user's GPS/provided address and configures default flat-rate shipping or local delivery zones.
- *Mobile UX (375px)*: A single glassmorphism card: "We've set up 8% Sales Tax for New York and $5 Local Delivery. Sound good? [Confirm] [Edit]".

**Implementation Prompt**: Implement an AI-driven setup step in the onboarding flow where the Finance Agent automatically generates shipping zones and tax profiles based on the user's location and business type, requiring only a one-tap confirmation on mobile.

**Priority**: P0
**Estimated Scope**: Large
