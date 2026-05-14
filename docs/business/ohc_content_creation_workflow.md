# OHC Content Creation Workflow

This document defines the standard operating procedure (SOP) for creating new help content for the One Human Corp platform.

## Step 1: Request & Triage

*   **Source**: Content requests can originate from Product Managers (new feature launches), Support Agents (frequent user questions), or automated analytics (high volume of zero-result searches).
*   **Action**: A ticket is created in the content backlog detailing the user need, target persona, and the proposed scope of the article or video.
*   **Triage**: The Content Lead reviews the ticket, assigns priority, and determines the most appropriate format (Tooltip, Walkthrough, Article, Video).

## Step 2: Drafting

*   **Action**: The assigned writer drafts the content following the OHC Plain Language Guide.
*   **Key Focus**: Emphasize the "why" before the "how." Use clear, active voice.
*   **Review**: The draft is peer-reviewed for adherence to tone and style guidelines.

## Step 3: Technical Review

*   **Action**: A subject matter expert (SME) or engineer reviews the drafted content to ensure technical accuracy.
*   **Verification**: The SME confirms that the instructions match the current state of the UI and that no critical steps are missing.

## Step 4: Asset Creation (If Applicable)

*   **Action**: If the content requires a video tutorial or complex diagrams, these assets are produced simultaneously.
*   **Guidelines**: Videos must follow the portrait-optimized, under-90-seconds rule.

## Step 5: Integration

*   **Action**: The approved text and assets are integrated into the codebase (e.g., updating `HelpContent.ts`, adding a new Tooltip ID to `TooltipRegistry.tsx`).
*   **Testing**: The changes are tested locally to ensure formatting is correct and links function properly.

## Step 6: Deployment & Announcement

*   **Action**: The content changes are included in the next deployment cycle.
*   **Communication**: Significant new documentation or video series should be highlighted in the internal release notes and, if appropriate, in the user-facing "What's New" section.
