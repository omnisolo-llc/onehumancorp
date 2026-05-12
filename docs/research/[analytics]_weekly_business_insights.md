# Title: Plain-Language Weekly Insight Agent

## Problem Statement
Small business owners often feel overwhelmed by complex analytics dashboards filled with charts, graphs, and metrics they don't understand. They want to know how their business is doing without needing a degree in data science.

## Research Report
- **User Feedback:** Reviews of existing platforms often mention that analytics dashboards are "too complicated" or "hard to read on mobile." Owners just want to know if they are making money and what they should do next.
- **Value Proposition:** An agent that translates complex data into simple, actionable text messages (e.g., "You made $400 this week. Your top seller was the chocolate cake. You should make more for next weekend.") makes business intelligence accessible to everyone.

## Design Doc
- **Core Entity Types:** Business Metrics, Insight Report.
- **Key Relationships:** The agent analyzes Business Metrics (sales, inventory, traffic) to generate a plain-language Insight Report.
- **Mobile UX Flow (375px first):**
    1. User opts-in to weekly SMS/Push notification updates.
    2. Every Friday evening (or customized time), the user receives a brief message summarizing the week's performance.
    3. Tapping the notification opens a slightly more detailed, but still plain-language, view in the app.

## Implementation Prompt
- **User-Facing Outcome:** Instead of navigating a complex dashboard, the user receives a simple, conversational weekly update explaining their business performance and offering actionable advice.
- **Critical User Journey (CUJ):**
    1. User enables weekly insights.
    2. At the end of the week, the agent analyzes all sales and traffic data.
    3. The agent generates a short, encouraging, and informative summary.
    4. The user receives the summary via push notification or SMS.
- **Acceptance Criteria:**
    - Agent accurately summarizes key metrics (revenue, top products).
    - Language is simple, conversational, and non-technical (8th-grade reading level max).
    - Agent provides at least one actionable recommendation (e.g., "Consider restocking X").

## Priority
P2

## Estimated Scope
Small
