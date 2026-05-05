# Issue Brief: 100% Mobile-First Management Dashboard

## Problem Statement
Competitors like Shopify have mobile apps, but they are often watered-down versions of the desktop experience, making complex tasks (like inventory management or workflow edits) frustrating or impossible without a laptop ("Mobile Gaps" - 42%).

## Research Report
- **Competitor Audit**: Shopify and Wix dashboards are heavily desktop-first. The mobile apps are often criticized for crashing or hiding essential menus.
- **Pain Point Mapping**: "Mobile Gaps" (42%) and "Operational Fatigue" (68%).
- **Persona Context**: Fatima (Food Cart) and Carlos (Handyman) operate entirely from their phones while on the move. They need full platform functionality in their pocket.

## Design Doc
- **Core Principle**: If it can't be done on a 375px screen, it's not a feature.
- **UI Architecture**: Slint-based native UI.
- **Key Components**:
  - Unified Inbox (Messages from all channels).
  - "Action Required" Feed (Agent-queued tasks).
  - Quick-edit Inventory list with large touch targets (>= 44x44px).
- **Progressive Disclosure**: Default to 'Simple mode', hide advanced settings under a toggle.

## Implementation Prompt
Refactor the main management dashboard in Slint to guarantee full functionality on a 375px width screen without horizontal scrolling. Implement the "Action Required" feed as the primary landing view, prioritizing AI-generated tasks. Ensure all touch targets adhere to the >= 44x44px constraint and utilize the Progressive Disclosure Pattern for advanced settings.

## Priority
P0

## Estimated Scope
Medium
