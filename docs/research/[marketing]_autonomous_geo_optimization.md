# [Marketing] Proactive GEO Agent: Autonomous Search Optimization

## Problem Statement
Traditional SEO is dead for small businesses. Users like **Carlos (Handyman)** don't have time to research keywords. They just want to be the answer when someone asks ChatGPT, "Who is a reliable handyman in Austin?" Current platforms provide "SEO Checklists" which are just more work for the owner.

## Research Report
- **Competitor Gap**:
    - **GoDaddy Airo**: Generates meta tags but doesn't optimize for AI crawlers.
    - **Durable**: Has a "GEO" score but it's a static assessment.
    - **Wix**: "SEO Wiz" is a step-by-step guide (manual work).
- **Trend**: "Generative Engine Optimization" (GEO) is the shift from keyword density to "cite-ability" by LLMs.
- **Evidence**: Reddit r/smallbusiness users frequently complain that "SEO feels like a black art" and "hiring an agency is too expensive."

## Design Doc
- **Architecture**:
    - **Tool**: Enhance `generative_visibility` from a "score" tool to an "optimization" tool.
    - **Entity**: `VisibilityMission` in `agent_missions`.
    - **Flow**: `MarketingAgent` periodically scans the site -> Runs `generative_visibility` -> Identifies missing Schema.org or descriptive gaps -> Proposes a "Visibility Boost" (code update to JSON-LD or meta content).
- **Mobile UX (375px)**:
    - Notification: *"The Promoter updated your site's 'AI visibility'—you are now 25% more likely to be recommended by ChatGPT."*
    - "Vibe" check: A simple "Before/After" toggle showing the descriptive changes.
- **AI Integration**: `MarketingAgent` (The Promoter) uses `websearch` to see how competitors are described and `write` to update the tenant's site metadata.

## Implementation Prompt
**Outcome**: Transform the `MarketingAgent` from a reactive tool to a proactive optimization engine that autonomously improves the business's visibility in Generative Search (GEO).
**Critical User Journey**:
1. `MarketingAgent` triggers a scheduled mission.
2. Agent analyzes the current storefront content using the `generative_visibility` logic.
3. Agent identifies that the business lacks "Austin-specific" qualitative descriptions.
4. Agent drafts an update to the site's metadata/schema.org JSON-LD.
5. User receives a weekly "Visibility Report" showing the improvement.
**Acceptance Criteria**:
- Must result in an actual update to the storefront's structured data (simulated or real).
- Must provide a plain-language explanation of *why* the change helps ("Help ChatGPT find you").
- Priority on local discovery (Persona: Carlos).

## Priority
P1

## Estimated Scope
Small/Medium
