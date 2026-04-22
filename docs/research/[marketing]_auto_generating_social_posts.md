# [marketing] Auto-Generating Social Posts

## Title
AI Agent for Autonomous Social Media Content Generation

## Problem Statement
Small business owners struggle to consistently post on social media to drive organic traffic. Creating content, writing captions, and scheduling posts is a full-time job they don't have time for.

## Research Report
*   **Competitor Analysis**: GoDaddy Airo offers basic AI generation but lacks ongoing autonomous scheduling. Most users rely on separate tools like Buffer or Hootsuite combined with ChatGPT.
*   **User Need**: An "invisible" marketer that automatically creates engaging posts (e.g., "Highlight of the week", "New item alert") using existing product photos and posts them to connected social accounts.

## Design Doc
*   **Architecture**:
    *   Cron-triggered AI jobs based on tenant's configured posting cadence.
    *   LLM prompt uses product catalog, recent reviews, or promotional context.
    *   Integration with Meta Graph API (Instagram/Facebook) and TikTok API.
*   **UI Wireframes**:
    *   "Marketing Hub" -> "Auto-Pilot" toggle.
    *   Connect Social Accounts button.
    *   Review/Approve queue (optional, can be fully auto).

## Implementation Prompt
Build the backend scheduling and generation pipeline for automated social media posts. The system should periodically select a product or recent positive review, use the LLM to generate a caption with relevant hashtags, and publish it to connected social media APIs. Provide a simple UI for the user to connect accounts and enable the feature.

## Priority
P1

## Estimated Scope
Medium
