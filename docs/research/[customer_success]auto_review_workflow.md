# [customer_success] AI Customer Success "Auto-Review" Workflow

**Problem Statement**: Business owners forget to ask for reviews, requiring expensive third-party apps on Shopify.

**Research Report**: Yotpo and Loox cost $15-$30/mo on Shopify. OHC must provide this natively as part of the "Customer Success" department.

**Design Doc**:
- *Trigger*: Order marked as 'Fulfilled' or Appointment 'Completed'.
- *AI Agent (Customer Success)*: Waits 48 hours, drafts a personalized SMS/Email asking for a review based on the specific item bought.
- *Mobile UX (375px)*: Dashboard notification: "The Ambassador drafted 3 review requests. Tap to send."

**Implementation Prompt**: Implement a background pg_queue job that triggers 48 hours after order fulfillment. The Customer Success agent generates a personalized review request payload. Expose a simple approval UI on the mobile dashboard.

**Priority**: P1
**Estimated Scope**: Small
