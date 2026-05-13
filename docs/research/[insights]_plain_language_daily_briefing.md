# Plain Language Daily Business Briefing

## Problem Statement
Traditional business analytics dashboards are fundamentally designed for data scientists and marketing professionals, not for bakers, plumbers, and handymen. Presenting a line chart detailing 'Conversion Rate' or 'Bounce Rate' to these users frequently induces anxiety and confusion. SMB owners harbor much simpler, more fundamental questions: 'Did I make any money yesterday?' and 'What specific actions do I need to take today to keep things running?'

## Research Report
Shopify's complex analytics dashboard is frequently cited in usability studies as a major source of overwhelm for new users. They genuinely do not understand what 'Sessions by traffic source' means in a practical sense, nor do they know how to adjust their operations based on that information. In our qualitative persona research, users like Carlos (Handyman) and Fatima (Food Cart) explicitly stated they just want a text-message-style summary of their business health, devoid of technical jargon.

## Design Doc
### Architecture Vision
- **Entities**: DailyBriefing, BusinessMetric, ActionableInsight.
- **UX Flow**:
  1. Every morning at precisely 8:00 AM local time, the user is presented with a 'Daily Briefing' card pinned to the top of the app interface.
  2. The text reads clearly: 'Good morning! You made $450 in revenue yesterday. You have 3 open orders to fulfill today. Your new cake post on Instagram received 50 likes. You should probably order more flour soon based on upcoming bookings.'
- **Mobile UX**: Relies on large, highly legible typography. It explicitly avoids complex charts, graphs, or heatmaps. The focus is entirely on narrative text delivery.
- **Agent Integration**: An Analyst Agent executes queries against the database overnight, synthesizes the complex metrics, and utilizes an LLM to generate a conversational, easy-to-read summary.

## Implementation Prompt
**Outcome**: Design a feature to replace or significantly supplement traditional chart-based dashboards with a daily, plain-text narrative summary detailing the business's health and highlighting required operational actions.
**Critical User Journey**:
1. The user opens the application first thing in the morning.
2. The user reads a concise, 3-to-4 sentence summary of yesterday's financial performance and today's outstanding tasks.
**Acceptance Criteria**: The generated summary must be consistently written at an 8th-grade reading level. It must strictly avoid technical e-commerce or marketing jargon (for example, utilizing the term 'visitors' instead of 'unique sessions').

## Priority
P1

## Estimated Scope
Medium
