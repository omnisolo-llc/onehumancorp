# Tooltip Registry System

*Note: This document describes the internal registry system for managing in-app tooltips. Content creators and support agents use this to update help text without modifying code.*

## Overview
Every non-obvious UI element in the app has a contextual tooltip. On desktop, this is triggered by a hover state. On mobile, this is triggered by a long-press.

The Tooltip Registry is a centralized mapping of `ui_element_id` to `plain_language_text`. This allows the documentation team (or AI Agents) to improve tooltip clarity on the fly.

## Design Constraints
- **Max Length:** 2 sentences strictly enforced.
- **Tone:** Plain language, 8th-grade reading level. No jargon.
- **Formatting:** No markdown or bold text inside the tooltip bubble.
- **Viewport:** Must render cleanly on a 375px mobile screen without overflowing.

## Registry Data Model

```json
{
  "tooltip_id": "string (UUID)",
  "ui_element_id": "string (e.g., 'btn-hire-agent')",
  "screen_context": "string (e.g., 'dashboard', 'settings')",
  "content": {
    "en": "string (Max 2 sentences)"
  },
  "last_updated": "timestamp",
  "updated_by": "user_id or agent_id"
}
```

## Core Tooltip Content Map (v1.0)

| UI Element ID | Screen | Tooltip Text |
|---------------|--------|--------------|
| `btn-hire-agent` | Agents | Hire a digital worker to help with customer support or marketing. They work 24/7. |
| `toggle-pause-store` | Settings | Temporarily hide your store from the public. You can turn it back on anytime. |
| `input-flat-rate` | Shipping | Charge the exact same shipping price for every order, no matter how heavy it is. |
| `btn-issue-refund` | Orders | Return money to the customer's credit card. This usually takes 3 to 5 days. |
| `toggle-2fa` | Security | Send a text message code to your phone every time you log in. This stops hackers. |
| `metric-gross-sales` | Dashboard | The total amount of money collected today, before fees and taxes are taken out. |
| `metric-net-sales` | Dashboard | The money you actually keep today, after fees and taxes are removed. |
| `input-stock-qty` | Products | How many of this item you have ready to sell right now. |
| `btn-sync-catalog` | Products | Update your store to match your latest inventory numbers. |

## Updating the Registry
The registry is loaded into the frontend client on boot. To update a tooltip, simply edit the core JSON registry file. Changes will propagate to all clients on the next refresh.

## Extended Tooltip Registry (v1.1 Additions)

| UI Element ID | Screen | Tooltip Text |
|---------------|--------|--------------|
| `btn-export-csv` | Analytics | Download a spreadsheet of all your sales data. You can open this in Excel or Google Sheets. |
| `toggle-abandoned-cart` | Marketing | Automatically email people who left items in their cart without buying. This helps recover lost sales. |
| `input-affiliate-rate` | Marketing | The percentage of the sale you will give to the influencer who brought in the customer. |
| `btn-connect-stripe` | Settings | Connect your bank securely so you can receive money from credit card purchases. |
| `select-tax-nexus` | Settings | Choose the states where you are legally required to collect sales tax from your customers. |
| `btn-test-payment` | Settings | Run a fake transaction to see exactly what the checkout process looks like for your customers. |
| `toggle-dark-mode` | Profile | Switch the app's colors to dark grey and black. This is easier on the eyes at night. |
| `metric-conversion-rate` | Analytics | The percentage of people who visited your store and actually bought something. |
| `btn-generate-report` | Analytics | Create a beautiful PDF summary of your business performance for the month. |
| `input-custom-domain` | Settings | Use your own web address (like www.mycoolstore.com) instead of the default one we give you. |
| `toggle-maintenance-mode` | Settings | Put up a "Be Right Back" sign on your store while you make big changes to your products. |
| `btn-sync-quickbooks` | Integrations | Automatically send all your sales data to QuickBooks so your accountant is happy. |
