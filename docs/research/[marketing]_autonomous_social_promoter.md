# Issue Brief: Autonomous Social Promoter

## Problem Statement
Small business owners know they need to post on social media to drive sales, but they suffer from "marketing paralysis." They lack the time or copywriting skills to create engaging posts for every new product, service, or promotion. As a result, their social channels go dormant, and they lose a primary source of customer acquisition.

## Research Report
- Consistent social posting is the #1 organic growth driver for micro-businesses.
- Existing tools (like Buffer or Hootsuite) require the user to write the content; they only help with scheduling.
- **Opportunity:** OHC can close the loop by automatically generating the content when a business event occurs, turning the "Marketing & Advertising" AI department into a proactive employee.

## Design Doc
### High-Level Architecture
- **Trigger:** System events such as `ProductAdded`, `ServiceCreated`, or `PositiveReviewReceived`.
- **Agent Integration:** The Marketing Agent listens for these events via the background job queue.
- **Content Generation:** The agent uses context (e.g., the new product's photo and description) to generate platform-specific copy (Instagram caption with hashtags, Facebook update, brief Twitter announcement).
- **Approval Flow:** The drafted posts are placed in a queue for the business owner to review before publishing via connected social APIs.

### Mobile UX Flow (375px First)
- **Trigger Event:** User adds a new "Vegan Chocolate Cake" to their catalog.
- **Immediate Feedback:** A toast appears: "The Promoter is drafting social posts for this item."
- **Review Screen:** A carousel showing the drafted posts for different platforms.
- **Actions:** The user can tap "Post Now", "Schedule for Tomorrow", or edit the text.

## Implementation Prompt
Implement the backend listener for catalog update events (`ProductAdded`, etc.) that triggers the Marketing AI Agent. The agent should draft social media content and store it in a pending state. Build the Flutter UI component (optimized for 375px width) that displays these pending social posts to the business owner, allowing for easy review and 1-tap approval/scheduling.

## Priority
P2

## Estimated Scope
Medium
