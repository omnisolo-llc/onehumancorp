## [Email Marketing] Issue Brief: AI-Generated Customer Broadcasts

**Title**: Scout 🔍: Integrate Resend for AI-Powered Email Marketing
**Problem Statement**:
Business owners like Priya want to notify their existing customers about new stock or holiday sales. Traditional tools like Mailchimp are too complex and require manual template design, list management, and campaign scheduling.
**Research Report**:
- **Tool**: Resend.
- **Evaluation**: Resend provides a developer-friendly, reliable email API. Instead of giving users a complex drag-and-drop builder, OHC can use the "Marketing" AI agent to generate beautiful HTML emails based on a simple text prompt from the user.
- **Ease of Use**: Zero-friction. The user types "Tell my customers about the new summer dress collection," and the AI generates the subject line, body, and inserts product photos automatically.
- **Pricing**: Resend charges around $20/mo for up to 50k emails, very economical to bundle into an OHC premium tier.
- **Cloud vs. Standalone**: Cloud mode uses OHC's centralized Resend account. Standalone mode requires the user to input their own SMTP credentials.
**Design Doc**:
- "Marketing" tab -> "Send a Broadcast".
- User provides a 1-sentence prompt.
- The AI Agent generates a responsive HTML email preview.
- User clicks "Send to all customers".
- The system chunks the customer list and sends via the Resend API.
**Implementation Prompt**:
Create a feature where the user can prompt the AI to draft an email blast. Use the business's product catalog to enrich the email. Provide a preview UI. Once approved, queue the emails to be sent out via the Resend API to all opted-in customers, handling rate limits and basic bounce tracking.
**Priority**: P2
**Estimated Scope**: Medium
