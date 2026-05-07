# Tool Integration Research Q4

## Overview
This report contains research on 7 integration categories essential for small business owners, evaluating tools that solve real pain points for non-technical users in both Cloud and Standalone environments.

## Research Summaries

### 1. Social Media Integration
*   **Tool:** Meta Graph API
*   **Target Persona:** Businesses managing communications across FB, Insta, and WhatsApp.
*   **Summary:** Essential for consolidating messages. Provides massive reach but Meta's review and OAuth setup are a major hurdle for Standalone, requiring cloud proxies. P0 priority.

### 2. Calendar & Scheduling
*   **Tool:** Cal.com
*   **Target Persona:** Service-based businesses playing "email tag".
*   **Summary:** Powerful open-source scheduling. Highly customizable and developer-friendly. Simple enough for users if we abstract the advanced features. P1 priority.

### 3. Email Marketing
*   **Tool:** Resend
*   **Target Persona:** Businesses wanting to run simple email campaigns.
*   **Summary:** Extremely developer-friendly with a generous free tier. The user interface must simplify the experience compared to complex tools like Mailchimp. P2 priority.

### 4. Payment Processing
*   **Tool:** Mercado Pago
*   **Target Persona:** LATAM businesses needing localized payment methods.
*   **Summary:** Crucial for LATAM markets due to support for local methods like Pix and OXXO. Standalone integration is challenging for asynchronous payment callbacks. P1 priority.

### 5. Shipping & Logistics
*   **Tool:** Shippo
*   **Target Persona:** Businesses shipping physical goods.
*   **Summary:** Automates rate calculation and label generation across carriers. Excellent fit for both Cloud and Standalone as long as physical dimensions are configured. P2 priority.

### 6. SMS & Notifications
*   **Tool:** Twilio
*   **Target Persona:** Businesses needing reliable appointment reminders to prevent no-shows.
*   **Summary:** Industry standard but A2P 10DLC compliance is a massive headache for micro-businesses. We must abstract this setup entirely. P1 priority.

### 7. Video Conferencing
*   **Tool:** Google Meet
*   **Target Persona:** Businesses offering online consultations or classes.
*   **Summary:** Ubiquitous and free. Easy for Gmail users, but OAuth configuration in Standalone mode requires careful handling so users don't need a GCP project. P2 priority.

## Next Steps
- Implementation engineers should review the generated briefs in `docs/research/`.
- Ensure Standalone edge cases (especially webhooks for Meta and Mercado Pago) are addressed in the architecture design.
