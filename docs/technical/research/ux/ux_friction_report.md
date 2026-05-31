# OHC UX Friction Report & Remediation

## Executive Summary

During the end-to-end customer journey testing, we identified critical UX friction points that fail the Aesthetic Excellence Mandate and slow down discovery.

## Friction Points Identified
1. **API Key Input Field Lack of Visibility Toggle**: The `AgentHireWizardScreen` and `SetupWizardScreen` use obscureText for API keys without a toggle. This creates friction when pasting keys and verifying they are correct.
2. **Missing Glassmorphism Aesthetics**: Several core setup screens appear visually flat and non-premium.
3. **Missing Tooltips & Semantics**: Important wizard actions lack accessibility labels and informative tooltips.

## Remediation Applied
- Added Semantics tags and Tooltips to buttons in SetupWizardScreen and AgentHireWizardScreen.
- Added a visibility toggle to the `_minimaxKeyCtrl` input field to reduce friction.
- Elevated the visual hierarchy with glassmorphism `BackdropFilter` utilizing the exact `blur(20px) saturate(200%)` token per design mandates.

## Visual Proof

The original screenshot artifacts referenced by this report were not committed with the documentation source. Re-run the UX capture flow and add the images under `docs/technical/research/ux/screenshots/2026-03-30/` before restoring image embeds.
