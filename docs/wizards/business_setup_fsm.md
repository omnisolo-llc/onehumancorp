# Business Setup Wizard State Machine

## Overview
The OHC Business Setup Wizard is a multi-stage flow designed to take a user from an initial idea to a live, AI-generated storefront in under 10 minutes.

## States & Transitions

| Current Status | Wizard Step | Description | Next Status/Step | Trigger |
|----------------|-------------|-------------|------------------|---------|
| `onboarding` | N/A | **Goal Selection**: User chooses what they are building (Products, Services, or Work). | `idle` / Step 1 | Option selected |
| `idle` | 1 | **Business Info**: User enters Business Name (min 3 chars) and Category (min 5 chars). | `idle` / Step 2 | "Next" clicked (validated) |
| `idle` | 2 | **Vibe Selection**: User selects a visual/tonal vibe (e.g., Professional, Friendly). | `idle` / Step 3 | Vibe selected + "Next" |
| `idle` | 3 | **AI Team Selection**: User selects AI agents and configures capabilities. | `idle` / Step 4 | Agents selected + "Next" |
| `idle` | 4 | **Final Details**: User reviews/edits the generated business bio. | `generating` | "Build Store" clicked |
| `generating` | N/A | **AI Architect**: System generates 3 storefront drafts in the background. | `selection` | Generation complete |
| `selection` | N/A | **Draft Picker**: User selects one of the 3 generated drafts. | `draft` | Draft selected |
| `draft` | N/A | **Customization**: User can reorder or edit blocks in the mobile-first editor. | `live` | "1-Tap Launch" clicked |
| `live` | N/A | **Success**: User sees their live URL, QR code, and growth loop options. | N/A | End of flow |

## Data Persistence
- All state changes are synced to `localStorage` for immediate local resume.
- High-level progress is synced to the backend via `/api/onboarding/state` to support cross-device resume.

## Design Standards
- **Typography**: Outfit (headings), Inter (body).
- **Glassmorphism**: 30px blur, 210% saturation, 0.65 opacity (light) / 0.7 (dark).
- **Corners**: 8px (controls), 16px (cards).
- **Motion**: 250ms entrance, 150ms exit, cubic-bezier(0.4, 0, 0.2, 1).
