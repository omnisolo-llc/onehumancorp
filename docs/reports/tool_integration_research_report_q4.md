# Tool Integration Research Q4

## 1. WhatsApp Business API
**Priority**: P1 | **Scope**: Large

**Research Summary**:
Small business owners receive customer inquiries on WhatsApp but struggle to manage them alongside other messages. Meta's official API is reliable but has strict opt-in rules. Requires OAuth/Meta Business Login to connect the account. OHC must handle incoming Webhooks and provide a unified inbox view for replies.

## 2. Calendly Integration
**Priority**: P2 | **Scope**: Medium

**Research Summary**:
Service-based businesses spend too much time going back and forth with clients to find a suitable meeting time. Calendly is user-friendly and standard. Integration via OAuth allows OHC to sync new bookings into the dashboard and auto-create/update customer records via webhooks or polling.

## 3. Mailchimp Integration
**Priority**: P1 | **Scope**: Medium

**Research Summary**:
Business owners collect customer emails but find it hard to keep their marketing lists in sync. Mailchimp is established and popular. Integration will automatically push new contacts created in OHC (via purchase or form) to a designated Mailchimp audience using standard API calls.

## 4. Mercado Pago
**Priority**: P2 | **Scope**: Large

**Research Summary**:
Small businesses in Latin America often cannot use Stripe or prefer local payment methods. Mercado Pago is dominant in LATAM. OHC must add it as an alternative payment provider option, redirecting customers to Mercado Pago at checkout and processing payment confirmation webhooks.

## 5. Shippo Integration
**Priority**: P1 | **Scope**: Large

**Research Summary**:
Online sellers spend hours manually copying addresses to carrier websites. Shippo aggregates multiple carriers into one API with friendly pricing. Integration allows merchants to click "Buy Shipping Label" on an order, confirm details, and download a PDF label directly from OHC.

## 6. Twilio SMS
**Priority**: P2 | **Scope**: Medium

**Research Summary**:
Businesses need to reach customers for reminders/updates via text. Twilio is standard and reliable. OHC must completely abstract Twilio's developer nature, exposing SMS as a standard communication channel in the unified inbox. Pay-per-message model requires careful billing handling.

## 7. Zoom Integration
**Priority**: P2 | **Scope**: Medium

**Research Summary**:
Service businesses manually create Zoom links and email them to clients for every session. Zoom OAuth is standard. Integration ensures that when a virtual appointment is scheduled in OHC with "Make it a Zoom meeting" checked, the API creates a meeting and adds the join URL to the calendar invite.