# Meta WhatsApp Cloud API Integration for Customer Success

## Title

WhatsApp Business Cloud API Integration for Automated Customer Success

## Problem Statement

Small business owners (like Maya the baker, Fatima the food cart operator, and Carlos the handyman) receive a massive volume of customer inquiries via WhatsApp—from order updates and pricing requests to booking confirmations and simple FAQs. Managing these conversations manually is overwhelming, leading to delayed responses, lost sales, and poor customer satisfaction. For non-technical owners, current WhatsApp integrations in traditional builders are either completely non-existent, highly technical to set up, or require expensive third-party tools. They need an invisible AI agent ("The Ambassador") to automatically handle WhatsApp conversations 24/7 without requiring them to install complex software or understand webhook APIs.

## Research Report

- **Market Demand:** WhatsApp is the most popular messaging app globally, especially in LATAM, EMEA, and APAC. It is the primary communication channel between small businesses and customers in these regions. E-commerce platforms like Shopify have an entire app store category dedicated to WhatsApp integrations, with top apps boasting thousands of positive reviews.
- **Competitor Analysis:** Shopify and Wix rely on third-party apps for WhatsApp integration. These apps often cost between $15 to $50/month and require technical setup (e.g., Twilio account creation, webhook configuration).
- **The Solution - Meta WhatsApp Cloud API:** Meta now offers a direct Cloud API for WhatsApp Business, eliminating the need for intermediary hosting or complex Twilio setups for basic use cases. It supports rich media, structured messages (buttons, lists), and webhooks.
- **Pricing & Viability:** The first 1,000 service conversations per month are free, which is more than enough for the vast majority of our target personas. Beyond that, the cost per conversation is region-dependent but generally affordable for small businesses.
- **Usability for Non-Technical Users:** The integration can be abstracted away by OHC. The user simply authenticates their WhatsApp Business account via Meta's embedded signup flow (OAuth), and the OHC platform automatically connects the "Customer Success" AI agent to the WhatsApp channel.
- **Persona Benefits:**
  - _Maya (The Home Baker):_ Customers can message her WhatsApp number, and the AI agent automatically replies to questions like "Do you do vegan cakes?" and provides a link to her OHC storefront.
  - _Carlos (The Handyman):_ Customers can send pictures of a broken pipe to his WhatsApp, and the AI agent can quote a price and provide a booking link.
  - _Fatima (The Food Cart Operator):_ Customers can pre-order via WhatsApp, and she receives a notification directly on her phone.

## Design Doc

- **Trigger:** A customer sends a message to the business's connected WhatsApp Business number.
- **Actions:**
  1. Meta sends a webhook payload to the OHC backend containing the message content (text, image, etc.).
  2. The OHC platform routes the message to the "Customer Success" AI agent.
  3. The agent accesses the business's context (inventory, pricing, FAQs) via its pgvector memory layer and tools.
  4. The agent generates a relevant, contextual reply.
  5. The OHC platform uses the WhatsApp Cloud API to send the agent's reply back to the customer.
- **User Experience (The Business Owner):**
  - In the OHC dashboard (or mobile app), the owner clicks "Connect WhatsApp".
  - They complete the standard Meta embedded signup flow to link their number.
  - The "Customer Success" agent is instantly active on that number.
  - The owner can view the conversation history in the OHC inbox and take over the chat manually if needed.

## Implementation Prompt

Integrate the Meta WhatsApp Business Cloud API to enable the Customer Success AI agent to handle WhatsApp conversations autonomously.

- **Outcome:** A business owner can connect their WhatsApp Business account via the OHC UI with zero technical setup. Once connected, incoming WhatsApp messages from customers are automatically answered by the "Customer Success" AI agent, utilizing the business's data and context. The owner can view these conversations and intervene if necessary.
- **Acceptance Criteria:**
  - Implement an OAuth/Embedded Signup flow for users to connect their WhatsApp Business account.
  - Establish a robust webhook endpoint to receive incoming WhatsApp messages securely.
  - Route incoming messages to the AI Agent framework, providing the agent with the necessary context and tools.
  - Format and send the AI's response back to the customer via the WhatsApp Cloud API.
  - Provide an inbox view in the UI where owners can read conversations and manually reply.
  - Ensure rate limits, free-tier boundaries, and error states are handled gracefully and communicated clearly to the owner.

## Priority

P0 (Critical) - WhatsApp is a fundamental communication channel for small businesses globally.

## Estimated Scope

Large
