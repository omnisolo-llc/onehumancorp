# Title
Native Email Campaign Manager (SendGrid)

# Problem Statement
Priya (Boutique Owner) wants to email her past customers when new stock arrives. External tools like Mailchimp are too complex and violate the Radical Simplicity rule. She needs an automated way to email customers natively within the OHC app.

# Research Report
- **Tool:** Twilio SendGrid Email API & Marketing Campaigns.
- **Target Persona:** Priya (Boutique Owner), Leo (Music Tutor)
- **Advantages:** Keeps the user within the OHC ecosystem. The AI agent can fully control the campaign without the user learning a third-party tool.
- **Risks:** Requires building list management and unsubscribe logic internally.
- **Pricing:** Very generous free tier (100 emails/day forever). Paid plans are competitive and scale well.
- **Compatibility:** Cloud (Centralized SendGrid account). Standalone (User provides API key).

# Design Doc
- **Integration Trigger:** OHC uses SendGrid under the hood. For marketing, the user simply asks the Promoter agent to "email all past customers about the summer sale".
- **User Flow:** The user reviews the AI-generated email draft and audience list, then clicks "Send".
- **Action Flow:** OHC compiles the HTML email and uses the SendGrid API to dispatch the messages. Delivery events (opens, clicks, bounces) are received via SendGrid Event Webhooks and displayed in the OHC Analytics dashboard in plain language.

# Implementation Prompt
Build a native email campaign management system. Utilize SendGrid for delivery. Allow the AI Marketing agent to create and queue email campaigns directly from the OHC database. Implement webhook handlers to track open and click rates so the Business Advisory agent can report on campaign success in plain language.
- **Acceptance Criteria:** User can create an email campaign. AI can generate content. Emails are delivered. Unsubscribe links work. Open rates are displayed.
- **Priority:** P1
- **Estimated Scope:** Large
