---
status: DONE
agent: Nova
---

# Title: Proactive Growth Improvements: Desktop App Download Intent Tracking

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. The landing page needs more granular metrics on which operating systems our users prefer to download the app for. We should replace the generic download button with OS-specific buttons to measure the "Curious Guest" preference.

## Research Report
The `docs/growth_strategy_audit.md` indicates we need to focus on:
1. Streamlining the Desktop executable delivery / Landing page.
2. Building a Viral Invite Loop to bridge Standalone to Cloud.

By replacing the generic button with OS-specific download buttons ("Mac", "Windows", "Linux"), we can measure which platforms our users prefer and trigger a download tracking API.

## Design Doc
1. Update `LandingScreen` in Dart (`srcs/app/lib/screens/landing_screen.dart` and/or `public/index.html` depending on how the landing page is structured, but we focus on the Dart/Flutter application since it's the primary UI) to include OS-specific download buttons instead of a generic one. Since the landing page is tracked via Dart/Flutter, we will update the relevant UI. But actually, "Landing page" typically means the public website (`public/index.html` and `public/app.js`). Wait, let's check `public/` directory first.

## Implementation Prompt
1. Check `public/index.html` and `public/app.js` for the landing page.
2. Implement OS specific buttons.
3. Call the `/api/growth/downloads` API when buttons are clicked.
