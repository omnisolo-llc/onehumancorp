# Social Media Integration Module

## Problem Statement
Small business owners struggle to manage messages from Instagram, Facebook, WhatsApp, and TikTok across different apps, leading to missed sales and slow responses.

## Research Report

**Market Context:**
Social media marketing is the use of social media platforms and websites to promote a product or service. Although the terms e-marketing and digital marketing are still dominant in academia, social media marketing is becoming more popular for practitioners and researchers.
Social media platforms such as Facebook, LinkedIn, Instagram, and Twitter, among others, have built-in data analytics tools that companies can use to track the progress, success, and engagement of social media marketing campaigns. Companies address a range of stakeholders through social media marketing, including current and potential customers, current and potential employees, journalists, bloggers, and the general public.
On a strategic level, social media marketing includes the management of a marketing campaign, governance, setting the scope (e.g. more active or passive use) and the establishment of a firm's desired social media "culture" and "tone".
Firms that use social media marketing can allow customers and Internet users to post user-generated content (e.g., online comments, product reviews, etc.), also known as "earned media", rather than use marketer-prepared advertising copy.

**Evaluated Tools:**

#### In-Depth Evaluation: Meta Business Suite
**Market Position**: Meta Business Suite is the undisputed behemoth for social media management, particularly because it owns Facebook, Instagram, and WhatsApp. For small businesses, it's often the first and only tool they use.
**Pricing**: Free to use, monetized via ads.
**Integration Approach**: We must use the Graph API. The primary challenge is Facebook's notoriously strict app review process. In Standalone mode, we need a mechanism to securely hold the long-lived Page Access Tokens in the SIPDB without requiring the user to re-authenticate frequently.
**Persona Impact**: Consolidating Instagram DMs and Facebook comments into OHC means Fatima doesn't have to switch between the Meta app and her order management system. It's a massive time-saver.
**Technical Feasibility (Cloud)**: High. Standard OAuth flow.
**Technical Feasibility (Standalone)**: Medium. Requires secure storage of long-lived tokens and handling of webhook drops if the local machine is offline.

#### In-Depth Evaluation: Hootsuite
**Market Position**: A legacy player in the aggregator space. Powerful but often considered overly complex and expensive for micro-businesses.
**Pricing**: Tiers start high (e.g., $99/mo), which might be prohibitive for early-stage OHC users.
**Integration Approach**: Hootsuite offers APIs, but integrating an aggregator into another platform (OHC) might be redundant. We should likely connect directly to the source networks instead.
**Persona Impact**: Likely too heavy for Fatima. Better suited for a dedicated marketing team.

#### In-Depth Evaluation: Buffer
**Market Position**: Known for its simplicity and excellent UX. A strong alternative for scheduling, though less focused on unified inboxing compared to Meta.
**Pricing**: Very SMB friendly, with free tiers and low-cost per-channel pricing.
**Integration Approach**: REST API is developer-friendly. We could integrate Buffer for outbound publishing while relying on direct webhooks for inbound messages.

## Design Doc
Integrate a unified inbox module that aggregates messages from the top 4 platforms via their respective OAuth and Webhook APIs. When a new message arrives, it triggers an event in OHC's Orchestration Hub, which creates a unified task or notification.

## Implementation Prompt
Create a unified messaging dashboard component that displays incoming messages from multiple social channels. Users should be able to reply directly from the dashboard, and the system should track response times.

## Priority
P0

## Estimated Scope
Large
