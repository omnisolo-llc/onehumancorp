# OHC Tool Integration Research Report [Q3]

This report details the investigation and evaluation of external tools to expand OneHumanCorp's (OHC) capabilities for small business owners in both Cloud and Standalone environments.

## Methodology
The research focused on tools that directly solve pain points for non-technical small business owners, emphasizing ease of use, integration feasibility, and support for dual deployment modes.

Detailed issue briefs have been generated and saved in the repository under `docs/research/`:

1.  **Email Marketing (`docs/research/email_marketing.md`)**: Evaluated Resend, Brevo, Klaviyo, and Kit. Selected Resend for API-first embedded experiences and Brevo for all-in-one SMB solutions.
2.  **Social Media Integration (`docs/research/social_media_integration.md`)**: Evaluated Manychat, Intercom, and MessageBird. Recommended direct Meta Graph API or MessageBird for a unified inbox.
3.  **Calendar & Scheduling (`docs/research/calendar_scheduling.md`)**: Evaluated Calendly and Cal.com. Strongly recommended Cal.com for its open-source nature and 'Atoms' UI components.
4.  **Payment Processing (`docs/research/payment_processing.md`)**: Evaluated Razorpay, Mercado Pago, and dLocal. Recommended building a payment abstraction layer to support regional gateways like Razorpay alongside Stripe.

## Key Findings & Strategic Recommendations

*   **API-First vs. All-in-One**: For OHC to maintain a cohesive user experience, integrating with API-first platforms (like Resend for email, Cal.com for scheduling, and MessageBird for omnichannel chat) is heavily preferred over iframe-embedding all-in-one platforms.
*   **The Standalone Challenge**: Tools like Cal.com that offer open-source/self-hosted versions are incredibly valuable for OHC's Standalone mode, ensuring users have access to scheduling without relying on cloud infrastructure.
*   **Global Reach**: Payment processing must move beyond Stripe to capture the global SMB market. An abstracted payment layer is a prerequisite for integrating regional leaders like Razorpay (India) and Mercado Pago (LATAM).

The accompanying issue briefs provide detailed problem statements, design docs, and implementation prompts ready for the engineering team.
