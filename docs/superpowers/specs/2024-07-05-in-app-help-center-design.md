# In-App Help Center Design

## Overview
We need to implement an In-App Help Center for the non-technical owners using OHC. This will include a searchable help portal, contextual tooltips, interactive walkthroughs, an AI-powered help chat, embedded video tutorials, API documentation for advanced users, and release notes/changelogs.

## Approach
1. **Help Center Portal**: A slide-out or full-page modal accessible from a "?" button, organized by categories (Getting Started, My Store, etc.).
2. **Contextual Tooltips**: Enhance the existing `TooltipRegistry.tsx` by providing a default registry that works even if the backend returns nothing (or ensure backend integration). Use plain language.
3. **Interactive Walkthroughs**: Enhance `Walkthrough.tsx` with guided step-by-step functionality (highlighting elements, speech bubble) as an overlay without blocking interactions.
4. **AI-Powered Help Chat**: A floating "Ask anything" button that connects to a Help Agent using the Next.js API route, streaming responses based on help content. Provide links to full articles.
5. **Video Tutorials**: A component embedding short videos (<90s) in a portrait-optimized player.
6. **API Documentation**: Use `swagger-ui-react` to provide an advanced API view, hidden from the main menu but accessible in an advanced section.
7. **Release Notes**: Parse `CHANGELOG.md` or a structured changelog endpoint to display "What's New".
