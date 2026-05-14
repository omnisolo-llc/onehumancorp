# Integrated Email Campaigns

**Problem Statement**: Exporting customer lists to Mailchimp is tedious and disconnects sales data from marketing efforts.

**Research Report**: Mailgun/Sendgrid are good for transactional, but we need campaign management. Integrating a simplified email builder directly linked to the customer CRM saves time. Must handle unsubscribe links and CAN-SPAM compliance. Cloud is better for sender reputation.

**Design Doc**: Email template builder in UI. Select customer segments from CRM. Send via backend email provider integration.

**Implementation Prompt**: Develop a simple email campaign sender that uses the existing customer list and tracks open rates.

**Priority**: P2
**Estimated Scope**: Medium
