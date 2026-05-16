# Title: Ayrshare Social Media Integration

## Problem Statement
Small business owners often struggle to maintain a consistent presence across multiple social media platforms (Instagram, Facebook, LinkedIn, X, etc.). Manually logging into each app to post updates or check messages is tedious and takes them away from core business operations. They need a unified way to schedule posts and view interactions without juggling passwords or multiple tabs.

## Research Report
*   **Overview**: Ayrshare provides a unified API to post and manage content across 13+ social networks including Facebook, Instagram, Twitter/X, and LinkedIn. It is designed specifically for SaaS platforms to offer social media features to their users.
*   **Ease of Use**: By abstracting the complex OAuth flows of individual networks, Ayrshare makes the end-user experience very simple. The business owner authorizes OHC once per network, and then manages everything from the OHC dashboard.
*   **Reputation**: Strong alternative to Hootsuite/Buffer for embedded use cases.
*   **Pricing**: Based on the number of "Social Profiles" (user accounts) managed. The Business Plan starts at $599/month for 30 profiles, dropping to $2.49/profile as volume scales over 500. This is usage-aligned.
*   **Environment (Cloud vs Standalone)**: Ayrshare is a Cloud-based API. It works perfectly in OHC Cloud mode. In Standalone mode, the local instance can call the API, but webhook callbacks (for comments/messages) will require the standalone instance to have a public IP or polling mechanism.
*   **AI Integration**: Ayrshare offers AI tools ("Max Pack") for post generation and analysis, complementing OHC's internal AI agent workflows.

## Design Doc
*   **Trigger**: A user writes a post in OHC's "Marketing" tab and schedules it, or a new comment arrives on a connected social account.
*   **Action**: OHC sends the post data and media to Ayrshare's API to publish across selected networks. OHC receives webhooks from Ayrshare when new comments arrive, displaying them in the unified inbox.
*   **User Interface**: A "Social Accounts" connection page. A compose box with a scheduling calendar. A unified feed showing published posts, engagement metrics, and incoming comments.

## Implementation Prompt
Integrate the Ayrshare API to enable social media publishing and monitoring. The user-facing outcome should allow business owners to connect their social accounts (e.g., Facebook, Instagram), schedule multi-platform posts from a single interface within OHC, and view incoming comments in their unified inbox. Ensure the integration securely manages Ayrshare's profile IDs mapped to OHC tenants.

## Priority
P1

## Estimated Scope
Large
