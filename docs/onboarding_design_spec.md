# OHC Onboarding Design Specification

## Overview
The OHC Onboarding Wizard is designed to be radically simple, aesthetically excellent, and AI-driven. It empowers non-technical users to launch a business in under 10 minutes.

## UX Patterns
- **One Question at a Time**: Minimize cognitive load by focusing on a single task per screen (inspired by Typeform).
- **Progressive Disclosure**: Only show necessary fields; use AI to pre-fill or handle complex configurations.
- **Fluid Transitions**: 250ms entrance animations, 150ms exit animations with `cubic-bezier(0.4, 0, 0.2, 1)` (inspired by iOS).
- **Glassmorphism UI**: Use translucent macOS-style materials for all containers.

## State Machine

### Steps:
1. **Business Type (Intake 1)**: "What do you do?"
   - Input: Text (e.g., "I sell cakes")
   - Transition: Next -> Step 2
2. **Business Name (Intake 2)**: "What's the name of your business?"
   - Input: Text (e.g., "Maya's Cakes")
   - Transition: Back -> Step 1, Next -> Step 3
3. **Niche & Details (Intake 3)**: "Tell us more about your niche."
   - Input: Text (e.g., "Custom wedding cakes and cupcakes")
   - Transition: Back -> Step 2, Submit -> Step 4 (AI Processing)
4. **AI Processing (Loading State)**: "Generating your business draft..."
   - Action: Call `/api/onboarding/intake`
   - Transition: Success -> Step 5
5. **Review & Customization (The Draft)**: "Review your setup."
   - Fields: First Product Name, Price, Template Selection, Domain Selection.
   - Transition: Back -> Step 3, Next -> Step 6
6. **AI Team Selection (New Step)**: "Choose your AI Team."
   - Selection: Toggle agents (The Manager, The Promoter, The Salesperson, etc.)
   - Transition: Back -> Step 5, Next -> Step 7
7. **Final Review & Launch**: "Ready to go live?"
   - Action: Call `/api/onboarding/start`
   - Transition: Success -> Step 8
8. **Live State**: "You're Live!"
   - Actions: Go to Dashboard, Preview Storefront.

## Design Tokens
- **Typography**: Outfit (Headings), Inter (Body).
- **Accent Color**: `#0066FF` (UniFi Blue).
- **Success Color**: `#34C759` (Apple Green).
- **Border Radius**: 8px (Controls), 16px (Cards).
- **Glass Effect**:
  - Light: `rgba(255, 255, 255, 0.65)`, Blur 30px, Saturate 210%.
  - Dark: `rgba(22, 22, 26, 0.7)`, Blur 30px, Saturate 210%.
