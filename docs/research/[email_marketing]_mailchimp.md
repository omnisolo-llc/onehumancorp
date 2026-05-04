### Title
Integrate Mailchimp for Customer Engagement and Email Marketing

### Problem Statement
Small businesses, such as boutiques or freelance service providers, struggle to maintain ongoing relationships with past customers. They collect email addresses during checkout or booking but lack an easy, professional way to send newsletters, product updates, or promotional offers without needing design skills or complex marketing software knowledge.

### Research Report
**Tool Evaluated:** Mailchimp (Intuit Mailchimp)
**Overview:** Founded in 2001, Mailchimp is a widely used marketing automation and email marketing platform. Initially launched as a paid service, it added a highly successful freemium option in 2009. It was acquired by Intuit in 2021 for approximately $12 billion. It handles billions of emails and has millions of active users globally.
**Key Features & Advantages:**
- Proven, robust email delivery infrastructure (handles transactional and marketing emails).
- Evolved into a full marketing platform, allowing businesses to track leads, build landing pages, and run social media ads.
- Freemium model provides an accessible entry point for new, budget-conscious small businesses.
- Strong brand recognition and intuitive template builders suited for non-technical users.
**Risks:** Mailchimp has experienced data breaches involving social engineering targeting their support teams (March 2022, January 2024), though this primarily affected a limited number of accounts. Furthermore, their historical changes to transactional email pricing (Mandrill) angered some developers, though this is less relevant to the end-user persona OHC targets.
**Ease of Use:** High. Designed specifically to be accessible to small business owners without marketing degrees.
**Pricing:** Freemium (free tier available, scaling up based on subscriber count).
**Deployment:** Cloud-native.

### Design Doc
**Integration Trigger:** From the OHC "Marketing & Advertising" or "Customer Success" department tab, the user selects "Start a Newsletter" or "Connect Email Marketing."
**Action:** OHC initiates an OAuth flow with Mailchimp. Once connected, OHC automatically syncs the tenant's customer list (from orders and bookings) to a designated Mailchimp audience.
**User Experience:**
- **Business Owner:** Does not need to manually export/import CSVs of customer emails. They simply log into OHC, see their subscriber count growing automatically, and can click a link to jump into Mailchimp to use their drag-and-drop template builder to send an update.
- **Customer:** Receives professionally formatted emails from the business owner, with working unsubscribe links (managed safely by Mailchimp).

### Implementation Prompt
Develop a Mailchimp integration service that acts as a bridge between the OHC customer database and Mailchimp's API.

**Acceptance Criteria:**
1. Provide an OAuth connection flow in the OHC UI to link a merchant's Mailchimp account.
2. Implement an automated background sync that pushes new customer emails (who have opted in) from OHC PostgreSQL to the connected Mailchimp Audience.
3. The sync must handle updates (e.g., if a customer changes their email or opts out via OHC, the status must reflect in Mailchimp).
4. Do not build an email template designer in OHC; offload the actual campaign creation to Mailchimp's UI, providing a deep link from the OHC dashboard.

### Priority
P1

### Estimated Scope
Medium
