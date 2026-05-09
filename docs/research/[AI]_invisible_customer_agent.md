# [AI] Invisible Customer Agent

## Problem Statement
Small business owners (like Carlos the handyman and Priya the boutique owner) spend hours every week answering the same repetitive questions ("Are you open today?", "Do you have this in medium?", "How do I book?"). When they get busy, they miss messages, leading to lost leads. Traditional chatbots require complex setup and decision-tree programming, which is beyond the technical comfort level of our target users.

## Research Report
- **Competitive Gap:** Shopify provides "Sidekick" for the merchant, but not for the end customer out-of-the-box. Third-party apps exist but cost $15-$50/mo and require setup.
- **User Pain:** Reddit r/smallbusiness threads frequently cite "answering DMs" as a top 3 time-waster.
- **Data Point:** Over 60% of pre-sale inquiries for SMBs are basic logistical questions (hours, location, inventory status, booking availability).

## Design Doc
- **Architecture Idea:** A background AI agent that subscribes to the unified messaging event stream. It reads incoming customer messages, checks the business's OHC knowledge base (inventory, hours, policies), and drafts a response.
- **UX Flow (Mobile 375px First):**
  1. User navigates to Settings -> AI Assistants.
  2. Toggles "Auto-Reply to Common Questions" to ON.
  3. No other configuration required. The agent learns from existing store data.
  4. In the Unified Inbox, messages handled by the agent have a subtle "Sparkle" icon indicating AI handled it.
- **Key Relationships:** Interacts with Messaging Service, Inventory Service, and Business Profile data.

## Implementation Prompt
Implement an invisible customer agent feature that automatically responds to routine customer inquiries. The agent should leverage the business's existing data (store hours, inventory, location) to answer questions without manual setup from the merchant. The user journey should consist of simply toggling the feature on, and being able to view AI-handled messages in their inbox. Ensure the solution is seamless and requires zero prompt-engineering or rule-configuration from the business owner.

## Priority
P0

## Estimated Scope
Large
