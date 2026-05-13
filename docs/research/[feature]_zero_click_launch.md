# [feature] Zero-Click Service Launch

## Problem Statement
Leo, a 22-year-old music tutor, needs a professional booking website but has zero design skills and no time to configure themes, layouts, or calendar integrations. He wants a site that just works immediately.

## Research Report
Platforms like Durable have proven that users respond incredibly well to 30-second website generation. However, legacy platforms like Squarespace and Wix still require users to manually drag-and-drop elements and configure backend integrations even after picking a template.

## Design Doc
*   **Architecture**: An orchestration engine that takes a single user prompt (e.g., "I am a guitar tutor in Austin") and autonomously provisions a database, configures a scheduling component (like Cal.com integration), writes all placeholder copy, and deploys the frontend.
*   **UX Flow**: The user enters their business type and location. A loading screen with real-time updates ("Configuring your calendar...", "Writing your about page...") is displayed. The user lands on a fully published, live site.
*   **Mobile UX**: The input form must be a single text field optimized for mobile typing.

## Implementation Prompt
Create an orchestration pipeline that generates a fully functional service business website (including a live booking component and generated copy) from a single text input prompt. The site must be published and live immediately without any drag-and-drop configuration required by the user.

## Priority
P0

## Estimated Scope
Large
