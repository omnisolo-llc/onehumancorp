# Issue Brief: Brevo (Sendinblue) Campaign Management

## 1. Problem Statement
**Context for the Small Business Owner:**
Business owners want to leverage their existing customer base by sending newsletters, seasonal promotions, and transactional emails, but lack a deeply integrated tool within OHC. The current workflow relies on exporting CSV files from OHC and importing them into external email providers like Brevo, which is tedious, error-prone, and leads to rapidly outdated contact lists, resulting in poor engagement and compliance risks.

Small businesses operate on razor-thin margins and strictly limited time. When a platform requires manual intervention—whether it's copying and pasting data between separate applications, or manually reconciling states—it introduces a point of failure. This proposed integration addresses a direct pain point identified in our user research, aiming to automate a critical segment of their daily operations.

## 2. Comprehensive Research Report

### 2.1 Tool Market Position & Historical Context
*Data sourced from public knowledge bases to understand the tool's maturity, market penetration, and technological underpinnings.*

**Wikipedia Extract (Abridged for Context):**
> Brevo, formerly Sendinblue, is a cloud-based software company that provides tools for marketing and relationship marketing. The company was founded in 2012 by Armand Thiberge and rebranded as Brevo in 2023, and offers a cloud-based marketing communication software suite with email marketing, transactional email, marketing automation, customer-relationship management, landing pages, Facebook ads, retargeting ads, SMS marketing, and SMS messaging and customer relationship management (CRM), and Customer Data Platform (CDP).
The company has eight offices globally, which are located in Paris, Delhi, Seattle, Berlin, Sofia, Toronto, New York and Vienna. The headquarters are located in the Paris office, which is also home to the customer service, marketing, product, and technical teams. There are currently 500,000+ customers using Brevo products worldwide. Features offered by Brevo software include A/B testing, report production, contact list management, and email heatmap.


== History ==
Brevo was founded in 2012 in Paris, France, under the name Sendinblue, by Armand Thiberge.  The company initially focused on providing email marketing and transactional email services for small and medium-sized businesses. The same year, the company opens its first offices in Paris, France and Noida, India.  Globally, the company has four offices: Paris, Delhi, Seattle, and Berlin. Sendinblue has over 500,000 active users in 160 countries. It is currently used by 180,000 businesses worldwide.
Sendinblue was chosen as one of 20 startups to watch in 2016 by Forbes magazine.
In October 2020, Brevo raised $160 million in Series B funding, one of the largest rounds for a French tech company at the time. The funding round was led by Bridgepoint, Bpifrance, and BlackRock, with the goal of expanding into new markets and developing additional tools such as CRM and automation features.
In 2023, Sendinblue changed its name to Brevo. Later that year, Brevo acquired mobile app push provider WonderPush and customer data platform (CDP) provider Octolis.


== Products ==
Brevo offers a cloud-based platform that combines marketing, sales, and customer relationship tools, primarily targeting small and medium-sized businesses as well as mid-market companies. Its core features include email marketing, SMS campaigns, marketing automation, transactional messaging, live chat, WhatsApp marketing, and a CRM that allows users to store contact data, create and manage deals and tasks, segment audiences, and automate follow-ups.
The platform also includes tools for push notifications (web and mobile) and cloud-based phone communication, introduced through the acquisition of Yodel.io in 2022. Brevo launched an AI-powered writing assistant in 2023 to help generate subject lines and content tailored to different tones and goals
Brevo also offers a Customer Data Platform (CDP) for unified data management and advanced segmentation, as well as a Commerce Suite designed for retail and e-commerce businesses. In 2024, it launched a built-in Loyalty Program tool that enables brands to create and manage customer rewards systems.
The platform integrates with a wide range of third-party applications, including WordPress, Shopify, Salesforce, and WooCommerce, WhatsApp, Instagram, Facebook Messenger, Google Meet, Zoom and more via plug-ins and APIs.


== Recognition ==
Brevo has received several industry recognitions for its growth and innovation in digital marketing services. In 2021 and in 2025, it was listed among the "Next40," a French government-backed index highlighting the 40 most promising French tech companies with global potential.
In 2023, Brevo surpassed $100 million in annual recurring revenue while remaining profitable.
In May 2025, Brevo achieved B Corp certification, with a B Impact Score of 130.5
In 2025, Brevo became a unicorn, with a valuation exceeding $1 billion, following a €500 million equity funding round ($583 million). Brevo plans to use these funds to accelerate its growth, invest in artificial intelligence, and expand its presence, particularly in the United States.


== References ==


### Related Market Context
#### Email marketing
Email marketing is the act of sending a commercial message, typically to a group of people, using email. In its broadest sense, any email sent to a potential

#### Marketing automation in email campaigns
Marketing automation in email campaigns refers to a numerous methods implemented in marketing for segmenting, targeting, scheduling, automating, and tracking

#### Email
Electronic mail (usually shortened to email; alternatively hyphenated e-mail) is a method of transmitting and receiving digital messages using electronic

#### Email spam
Email spam, also referred to as junk email, spam mail, or simply spam, refers to unsolicited messages sent in bulk via email. The term originates from

#### Email service provider (marketing)
An email service provider (ESP) is a company that offers email marketing or email services. An ESP may provide tracking information showing the status



### 2.2 Persona Fit & Use Case Analysis
**Target Persona:** E-commerce store owners, local brick-and-mortar shops, and service businesses focusing on customer retention and driving repeat sales through targeted campaigns.

The ideal user for this integration is not an IT professional. They evaluate software strictly through the lens of ROI (Return on Investment) and time saved. If the configuration process takes more than five minutes or requires understanding concepts like 'webhooks' or 'API keys' without extensive hand-holding, the feature will fail adoption. The design must abstract the technical complexity entirely.

### 2.3 Strategic Advantages
Integrating Brevo provides several distinct competitive advantages for the OHC platform:
*   **Operational Efficiency:** Brevo is highly affordable for SMBs compared to competitors like Mailchimp; it includes integrated SMS marketing capabilities which provides an expansion path; it offers robust marketing automation workflows that OHC users can leverage without OHC needing to build a complex workflow engine from scratch.
*   **Ecosystem Stickiness:** By embedding deeply into the tools the business owner already relies upon, OHC becomes an indispensable central hub rather than just another disconnected utility.
*   **Data Completeness:** Two-way integrations ensure that OHC maintains a holistic, accurate view of the customer relationship, which improves the quality of our internal reporting and AI-driven insights.

### 2.4 Risks & Mitigation Strategies
We must be clear-eyed about the technical and operational risks associated with this dependency:
*   **Vendor Lock-in and API Volatility:** Deliverability issues can arise if the user's OHC contact lists are of poor quality (e.g., scraped emails), which could negatively impact the platform's reputation; navigating strict and evolving compliance requirements (GDPR, CCPA, CAN-SPAM) regarding consent synchronization.
*   **Mitigation:** We must implement robust error handling, circuit breaker patterns, and graceful degradation. If Brevo experiences an outage, OHC must remain operational and clearly communicate the external failure to the user without crashing.

### 2.5 Pricing & Cost Implications
**Estimated Cost to User:** Brevo offers a generous free tier (up to 300 emails/day), making it highly accessible. Paid plans start at roughly $25/month for higher volumes. OHC integration is free.
It is imperative that the UI clearly communicates any third-party costs associated with using this tool prior to the user initiating the connection flow, avoiding 'gotcha' moments that erode trust.

### 2.6 Deployment Compatibility Matrix
**Evaluation of Architecture Constraints:**
*   **Cloud: Fully supported via API key integration and inbound webhooks for bounce tracking. Standalone: Supported, operating primarily via outbound API calls. Webhook reception may require user configuration of a tunneling service (like ngrok) or polling alternatives.**
Our commitment to the Standalone user base means we must carefully design the authentication and webhook flows to function (or gracefully degrade to polling) in environments without stable public IP addresses.

## 3. High-Level Design Document
**Architectural Approach (No implementation details):**
Deep integration with Brevo's marketing API (v3). OHC acts as the source of truth for customer data. A background worker process will continuously synchronize customer contacts (including custom attributes like name, email, purchase history tags, and VIP status) to specific dynamic lists within Brevo. Within the OHC dashboard, a marketing tab will utilize Brevo's reporting API to surface high-level campaign performance metrics (open rates, click-through rates, bounce rates) directly to the user. Crucially, opt-outs (unsubscribes) and hard bounces recorded in Brevo will trigger a webhook back to OHC to automatically update the customer's communication preferences, ensuring GDPR/CAN-SPAM compliance.

### 3.1 User Experience (UX) Flow
1.  **Discovery:** The user locates the Brevo integration card within the OHC 'App Store' or settings panel. The card clearly outlines the benefits and any associated costs in plain language.
2.  **Authorization:** The user initiates the connection. OHC handles the heavy lifting of the OAuth redirect or securely prompts for necessary credentials, accompanied by clear tooltips explaining *why* access is needed.
3.  **Configuration:** A simplified wizard guides the user through mapping any necessary fields (e.g., selecting which specific calendar to sync, or which specific email list to update).
4.  **Active State:** The integration runs silently in the background. The UI surfaces status indicators (e.g., 'Last synced 5 mins ago') and provides a clear, one-click mechanism to disconnect or troubleshoot the connection.

## 4. Implementation Prompt (For Engineering)
**Actionable Directives:**
Enable users to seamlessly connect their Brevo account via an API key input in the integrations settings. Provide an intuitive UI to map existing OHC customer segments (e.g., 'Recent Buyers', 'Lapsed Customers') to corresponding Brevo lists. Implement a reliable, real-time, one-way synchronization of customer data from OHC to Brevo. Finally, build a dashboard view within OHC that pulls and displays fundamental email campaign analytics, allowing the user to judge campaign success without leaving the platform.
*Note to Implementer: Your primary goal is stability and a zero-configuration feel for the end user. Abstract all technical jargon from the UI. Ensure thorough test coverage of the API failure states.*

## 5. Execution Parameters
*   **Priority Level:** P2
*   **Estimated Scope:** Medium
