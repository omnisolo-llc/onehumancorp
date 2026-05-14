# OHC Content Lifecycle & Localization Strategy

This document outlines the end-to-end lifecycle of help content within the One Human Corp ecosystem, from initial drafting to translation and eventual deprecation.

## 1. The Content Lifecycle

### Phase 1: Discovery & Drafting
*   **Trigger**: A new feature is developed, or analytics reveal a gap in existing content.
*   **Action**: A Technical Writer or Product Manager drafts the initial content following the OHC Plain Language Guide.
*   **Review**: Content is reviewed by the Product Owner for accuracy and by the UX team for tone and clarity.

### Phase 2: Integration & Deployment
*   **Action**: Approved content is added to `src/ui/next/src/components/help/HelpContent.ts` or the relevant Markdown file.
*   **Verification**: Ensure all links work, videos are embedded correctly, and search keywords are optimized.
*   **Deployment**: Content is shipped alongside application updates.

### Phase 3: Monitoring & Maintenance
*   **Action**: Continuously monitor Help Center analytics (Zero-Result Searches, Article Deflection Rate) and AI Agent Escalation Rates.
*   **Iteration**: Update articles based on user feedback and changing product features.

### Phase 4: Archival & Deprecation
*   **Trigger**: A feature is completely removed or significantly redesigned.
*   **Action**: Remove the corresponding help articles from active search and the `HelpContent.ts` store.
*   **Redirection**: If a feature is replaced, ensure the old search terms redirect to the new, relevant article.

## 2. Localization Strategy

As OHC expands globally, our help content must be accessible to non-English speaking small business owners.

### 2.1 Current State
Currently, all content is hardcoded in English within the `HelpContent.ts` file.

### 2.2 Future Architecture (Planned)
*   **Decoupling**: Move all text strings from `HelpContent.ts` and UI components into dedicated localization files (e.g., JSON or YAML format).
*   **Translation Management System (TMS)**: Integrate a TMS to streamline the translation workflow.
*   **Dynamic Loading**: Modify the Help Center component to asynchronously load the appropriate language file based on the user's locale settings.

### 2.3 Translation Guidelines
*   **Context is Key**: Provide translators with visual context (screenshots or staging links) to ensure accurate translations.
*   **Cultural Nuance**: Ensure translations account for cultural differences in tone, formatting, and business practices.
*   **Avoid Idioms**: Use clear, literal language that is easier to translate accurately.
