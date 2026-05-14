# Interactive Walkthroughs Architecture

*Note: This document details the technical implementation of in-app guided tours for the One Human Corp app.*

## Overview
We use a step-by-step highlight system to guide users through complex flows. We strictly avoid full-screen popups or blocking modals. Instead, the UI dims, a specific element is highlighted, and a friendly speech bubble points to the next action.

## Core Flows Supported
1. **Set up your store:** Guides the user to add their first product and connect a bank account.
2. **Accept your first payment:** Simulates a test transaction so the user sees exactly what the customer sees.
3. **Activate your AI Support Agent:** Guides the user to the Agents tab, selects the Support Agent, and sets a budget.

## Walkthrough State Machine
Walkthrough progress is saved to the user's database record. If they close the app halfway through a tour, they resume at the exact same step when they return.

```json
{
  "user_id": "string",
  "active_walkthrough": "store_setup",
  "current_step": 2,
  "completed_walkthroughs": ["welcome_tour"]
}
```

## UI Implementation Details
- **Dimming Layer:** `backdrop-filter: blur(5px)` with a 40% black overlay.
- **Highlight:** The target element is cloned using a React Portal and placed above the dimming layer, or the dimming layer uses a `clip-path` to cut out the target area.
- **Speech Bubble:** Attached to the target element using Floating UI to ensure it never goes off-screen, especially on mobile.

## Example Flow: "Activate AI Agent"
1. **Step 1:** Highlight the "Agents" tab in the bottom nav. Text: "Tap here to meet your digital employees."
2. **Step 2:** Highlight the "Customer Support" card. Text: "This agent answers questions for you while you sleep. Tap to view details."
3. **Step 3:** Highlight the "Hire" button. Text: "Tap Hire to activate them. Don't worry, we start them with a safe $5 budget."

Users can tap "Skip Tour" at any time. Dismissed tours can be restarted from the Help Center menu.

## Walkthrough: Setting Up a Discount Code
This tour helps users run their first promotion.
1. **Step 1:** Highlight the "Marketing" tab. Text: "Ready to run a sale? Tap Marketing."
2. **Step 2:** Highlight the "Create Discount" button. Text: "Tap here to make a new code."
3. **Step 3:** Highlight the "Code Name" input field. Text: "Give it a catchy name, like SPRING20."
4. **Step 4:** Highlight the "Percentage" input field. Text: "Enter the discount amount here. 20% is a great start!"

## Walkthrough: Reviewing Analytics
This tour introduces users to their performance metrics.
1. **Step 1:** Highlight the "Analytics" tab. Text: "Let's see how your business is doing. Tap Analytics."
2. **Step 2:** Highlight the "Total Sales" chart. Text: "This chart shows your total earnings over time. Up is good!"
3. **Step 3:** Highlight the "Top Products" list. Text: "These are your bestsellers. Keep these items in stock!"

## Best Practices for Walkthroughs
- **Keep it short:** Never exceed 5 steps per walkthrough.
- **Be skippable:** Always show a clear "Skip Tour" button.
- **Trigger contextually:** Don't show the "Discount Code" tour until the user actually has products to discount.

## Walkthrough: Activating the AI Marketing Agent
This tour helps users hire their first marketing assistant.
1. **Step 1:** Highlight the "Agents" tab in the bottom nav. Text: "Tap here to see available digital employees."
2. **Step 2:** Highlight the "Marketing Agent" card. Text: "This agent can write emails and plan sales. Tap to learn more."
3. **Step 3:** Highlight the "Hire" button. Text: "Tap Hire. You're the boss now!"
4. **Step 4:** Highlight the "Budget" input field. Text: "Set a small budget to start, like $10."
5. **Step 5:** Highlight "Confirm Hire". Text: "Your new marketer is ready to work!"

## Building Custom Walkthroughs (Internal)
The Documentation Team can create new walkthroughs without developer assistance.
1. Identify the CUJ (Critical User Journey) that users are struggling with.
2. Find the exact `ui_element_id` for each step of the journey.
3. Write short, plain-language text for each step (max 1 sentence).
4. Submit a JSON payload to the Walkthrough Registry detailing the steps and element IDs.
5. The system will automatically generate the highlight and speech bubble logic.

## Walkthrough Analytics
We track the success rate of every walkthrough to see if it is actually helping users.
- **Completion Rate:** The percentage of users who make it to the final step without hitting "Skip".
- **Action Rate:** The percentage of users who actually perform the final intended action (e.g., actually saving the new discount code).
- If a walkthrough has a completion rate under 50%, the Design team is automatically notified to review and simplify the flow.
