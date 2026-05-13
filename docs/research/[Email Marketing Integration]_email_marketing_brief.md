# Email Marketing Integration

## Problem Statement
Keeping track of customer emails in spreadsheets and sending manual updates is inefficient. Owners need an easy way to send promotions and newsletters to their customer list.

## Research Report

**Market Context:**
Email marketing is the act of sending a commercial message, typically to a group of people, using email. In its broadest sense, any email sent to a potential or current customer could be considered email marketing. It involves using email to send advertisements, request business, or solicit sales or donations. The term usually refers to sending email messages with the purpose of enhancing a merchant's relationship with current or previous customers, encouraging customer loyalty and repeat business, acquiring new customers or convincing current customers to purchase something immediately, and sharing third-party ads.

**Evaluated Tools:**

#### In-Depth Evaluation: Mailchimp
**Market Position**: The 800lb gorilla of email marketing for SMBs. High brand recognition.
**Pricing**: The free tier is shrinking, but still viable. Paid plans scale quickly with audience size.
**Integration Approach**: The key is two-way sync of the audience list. If a user unsubscribes via Mailchimp, OHC must update its internal customer record immediately to comply with CAN-SPAM. The API is extensive but rate limits apply.
**Persona Impact**: Allows Fatima to send a holiday promotion to all past customers directly from the OHC interface.

#### In-Depth Evaluation: Constant Contact
**Market Position**: Older demographic, known for excellent customer support. Often used by non-profits and traditional retail.
**Pricing**: Starts around $12/mo, scaling with list size.
**Integration Approach**: Similar to Mailchimp, requires robust list syncing and webhook handling for bounce/unsubscribe events.

#### In-Depth Evaluation: Sendinblue (Brevo)
**Market Position**: (Now Brevo). Known for combining email marketing with transactional emails (SMTP) and SMS at a competitive price.
**Pricing**: Volume-based pricing (per email sent, not list size), which is highly attractive for OHC users with large but inactive lists.
**Integration Approach**: Strong API. Excellent candidate for both marketing and transactional receipts within OHC.

## Design Doc
Integrate an email campaign manager linking OHC's customer database to an email provider API (like SendGrid or Mailchimp). Users create templates in OHC; the system segments lists and triggers bulk sends, tracking open/click rates via webhooks.

## Implementation Prompt
Develop an email campaign builder with drag-and-drop template creation. Include audience segmentation features and a dashboard to display campaign performance metrics (open rates, click-through rates).

## Priority
P1

## Estimated Scope
Large
