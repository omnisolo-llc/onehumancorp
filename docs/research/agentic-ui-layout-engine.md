# [Research] Agentic UI Layout Engine

## Problem Statement
Small business owners (like Maya the baker) frequently break their website's mobile layout when trying to customize the desktop view. This causes frustration and lost sales, as they lack the technical skills to debug CSS media queries or responsive grids.

## Research Report
Our deep dive into traditional builders (Wix, Squarespace) reveals a common flaw: allowing users to freely drag-and-drop elements on a desktop canvas inevitably breaks the mobile experience. Mobile-first builders exist, but often lack the power to generate a compelling desktop site. We need a system where the user edits *intent* and *content*, while the system handles the *presentation*.

## Design Doc
We propose a "Single-Source-of-Truth" Mobile-First Layout Engine.
- **Core Concept**: Users input structured data (Text, Image, Service Price).
- **Rendering**: The UI engine deterministically generates the 375px mobile view first.
- **Extrapolation**: The desktop view is generated based on predefined, unbreakable responsive rules applied to the mobile view.
- **AI Integration**: The "Marketing & Advertising" agent suggests content layouts based on the business type, but the underlying grid is locked to prevent user-induced breakage.

## Implementation Prompt
Implement a prototype of the Layout Engine where the user can input a "Hero Section" (Title, Subtitle, Background Image, Call to Action). The UI must automatically render a perfect 375px mobile view and a corresponding desktop view. The user must NOT be able to manually drag the button 5 pixels to the left. All layout adjustments must happen via predefined "Vibe" toggles (e.g., "Sleek", "Playful", "Corporate") which the AI applies globally. Acceptance criteria: Playwright tests prove that content injected into the engine always results in a layout that passes basic responsiveness checks (no horizontal scrolling at 375px).

## Priority
P0

## Estimated Scope
Large
