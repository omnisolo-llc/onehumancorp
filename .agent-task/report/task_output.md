# 🔍 Scout: Tool Integration Research Q4

## Executive Summary
This research report evaluates tools to expand One Human Corp's (OHC) capabilities for small business owners across 7 key categories: Calendar & Scheduling, Payment Processing, Social Media Integration, Email Marketing, Shipping & Logistics, SMS & Notifications, and Video Conferencing. The evaluations focus heavily on the "Grandmother Test" (usability for non-technical users) and compatibility with both Cloud (multi-tenant) and Standalone (local) execution modes.

## Comparative Table: Tool Integration Candidates

| Tool | Category | Target Persona | Cloud Mode | Standalone Mode | Pricing Estimate |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Cal.com** | Calendar & Scheduling | Consultants, Tutors | Fully Supported | Fully Supported | Free for Individuals, $12/user/mo Teams |
| **Mercado Pago** | Payment Processing | LATAM E-commerce | Fully Supported | Supported | Transaction-based |
| **Manychat** | Social Media Integration | Retail, E-commerce | Fully Supported | Supported via API | Free up to 1K contacts, $15/mo Pro |
| **Mailchimp** | Email Marketing | All SMBs | Fully Supported | Fully Supported | Free tier, scales with contacts |
| **Shippo** | Shipping & Logistics | E-commerce | Fully Supported | Fully Supported | Pay-as-you-go |
| **Twilio** | SMS & Notifications | Service Businesses | Fully Supported | Fully Supported | Pay-as-you-go (per message) |
| **Zoom** | Video Conferencing | Consultants, Tutors | Fully Supported | Fully Supported | Free 40m, $15.99/mo Pro |

---

## 1. Calendar & Scheduling: Cal.com

### Problem Statement
Small business owners spend too much time coordinating schedules via email ping-pong. They need a simple, self-serve booking link.

### Research Findings
Cal.com is an open-source scheduling platform with a generous free tier for individuals.
*   **Environment Compatibility:** Excellent for both Cloud and Standalone modes.
*   **Pros:** Free for individuals, built-in payments and video.
*   **Cons:** Team routing requires paid tier.

---

## 2. Payment Processing: Mercado Pago

### Problem Statement
LATAM small business owners struggle with online payments using US-centric providers. They need localized checkout options (like OXXO cash payments).

### Research Findings
Mercado Pago is the leading LATAM payment processor.
*   **Environment Compatibility:** Works seamlessly via API and Webhooks for Cloud; polling/webhooks for Standalone.
*   **Pros:** Absolute market dominance in LATAM, supports cash payments.
*   **Cons:** Checkout Pro redirects off-site.

---

## 3. Social Media Integration: Manychat

### Problem Statement
Managing customer inquiries across Instagram, Facebook, and WhatsApp manually leads to missed sales.

### Research Findings
Manychat automates chat flows and captures leads across Meta platforms.
*   **Environment Compatibility:** Fully supported via API/Webhooks.
*   **Pros:** Easy visual builder, deep Meta integration.
*   **Cons:** Focused on automation rather than a pure unified inbox.

---

## 4. Email Marketing: Mailchimp

### Problem Statement
SMBs need a reliable way to communicate with their customer base without dealing with spam compliance manually.

### Research Findings
Mailchimp is an industry-standard email marketing tool.
*   **Environment Compatibility:** Fully supported via REST API.
*   **Pros:** Massive brand trust, great templates.
*   **Cons:** Expensive as the list grows.

---

## 5. Shipping & Logistics: Shippo

### Problem Statement
E-commerce businesses waste time calculating shipping rates and manually printing labels.

### Research Findings
Shippo is a multi-carrier shipping API for generating labels and comparing rates.
*   **Environment Compatibility:** Fully supported via REST API.
*   **Pros:** Excellent API, deeply discounted rates.
*   **Cons:** International shipping can still be complex.

---

## 6. SMS & Notifications: Twilio

### Problem Statement
Many customers don't check email; businesses need SMS for urgent alerts and reminders to reduce no-shows.

### Research Findings
Twilio provides programmable APIs for global SMS delivery.
*   **Environment Compatibility:** Fully supported via REST API.
*   **Pros:** Extremely reliable, pay-as-you-go pricing.
*   **Cons:** Headless API; OHC must build the entire UI.

---

## 7. Video Conferencing: Zoom

### Problem Statement
Online service providers waste time manually creating and emailing video links.

### Research Findings
Zoom provides cloud video conferencing with an API to generate meetings.
*   **Environment Compatibility:** Fully supported via OAuth/API.
*   **Pros:** Massive brand recognition, reliable.
*   **Cons:** Free tier limited to 40-minute meetings.

## Conclusion
All 7 tools represent high-value integration targets that directly solve severe pain points for non-technical small business owners. Detailed issue briefs for implementation of each tool have been added to the `docs/research/` directory.
