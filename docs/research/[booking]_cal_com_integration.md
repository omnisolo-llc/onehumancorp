# Integrate Cal.com for Seamless Customer Booking

## Problem Statement
Small business owners like Maya (yoga instructor) and Carlos (local contractor) lose countless hours every week going back and forth with clients over text and email just to find a time to meet. They need a professional, reliable way for customers to book appointments directly on their website without feeling like they are being handed off to a clunky third-party app.

## Research Report
After evaluating several scheduling tools (including Calendly and Acuity), Cal.com stands out as the optimal choice. It is an open-source scheduling infrastructure that offers a generous free tier and robust white-labeling. This means small businesses can maintain their brand identity. Crucially for OHC, Cal.com supports both managed SaaS (Cloud) and self-hosted deployments, making it fully compatible with our Standalone (local, private) environments. It is highly regarded for its ease of use and flexibility.

## Design Doc
OHC will integrate Cal.com as a seamless booking block.
- **What triggers it**: The small business owner enables "Online Booking" in their OHC dashboard and connects their existing calendar (e.g., Google Calendar, Outlook).
- **What actions it takes**: OHC automatically creates the necessary booking links and event types in the background. When a customer books a slot, a webhook notifies OHC to update the owner's dashboard.
- **What the user sees**: The business owner sees a simple interface to manage their availability and upcoming appointments directly within OHC. Their customers see a beautiful, branded booking widget on the business's website.

## Implementation Prompt
Create a new "Booking & Scheduling" integration in the OHC platform. The feature should allow a non-technical business owner to turn on online booking with a single click and connect their personal calendar. Provide a drag-and-drop booking widget for their storefront. When a client books an appointment, it should instantly appear in the OHC dashboard's upcoming schedule. Ensure the entire flow is strictly white-labeled under the small business's brand—clients should never know a third-party tool is powering the experience.

## Priority
P1

## Estimated Scope
Medium
