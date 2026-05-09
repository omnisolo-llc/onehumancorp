# Integrate WhatsApp Business API for Unified Inbox
## Problem Statement
Small business owners often miss inquiries from customers messaging them on WhatsApp while they are busy serving clients or managing the shop. Checking a separate app constantly is stressful and leads to lost sales. They need all customer messages to land in one simple inbox.
## Research Report
The WhatsApp Business API (via providers like Twilio or Meta directly) allows for reliable message parsing and automated replies.
- **Ease of Use**: Once connected, the user never has to leave the OHC app.
- **Pricing**: Meta's direct API is pay-per-conversation, while Twilio adds a small margin. Generally very affordable for low volumes.
- **Reputation**: Industry standard, extremely reliable.
## Design Doc
When a customer sends a message to the business's WhatsApp number, OHC receives it and displays it in the Unified Inbox. The business owner replies from the Inbox, and OHC sends it back via WhatsApp.
## Implementation Prompt
Create a "Connect WhatsApp" button in the settings. Walk the user through authorizing their WhatsApp Business account. Once connected, new WhatsApp messages should appear in the "Inbox" tab alongside email. Replies typed in the Inbox should reach the customer's WhatsApp. It must support both Cloud and Standalone (with clear setup instructions for the local API keys).
## Priority
P0
## Estimated Scope
Medium
