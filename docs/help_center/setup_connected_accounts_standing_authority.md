# Setup, Connected Accounts, and Standing Authority

### 1. Setup and Connected Accounts
To run your business, your AI team needs access to specific tools. Currently, you can securely connect **Stripe** (for payments and invoicing) and **OpenAI** (for AI inference).
- **Security:** Your credentials are encrypted and locked to your specific business (tenant-bound). We never store credentials in plain text.
- **Unsupported Tools:** Connections to other tools (like Google Workspace) are currently disabled to ensure your data remains secure until those integrations are fully verified.

### 2. Standing Authority and Exceptions
Your AI team operates strictly within the policies you set.
- **Revocation:** As an owner or admin, you can revoke a connection at any time. When you revoke an account (e.g., Stripe), the system immediately blocks new requests, though requests already sent to the provider may finish.
- **Exceptions & Recovery:** If a connection becomes invalid, its status changes to "verification required." The AI team will pause workflows relying on that tool until you re-authenticate.

### 3. Cost, Evidence, and Budget Limits
- **Hard Spend Caps:** Before the AI team takes any action that costs money, it makes an atomic reservation against your budget. If your budget is exhausted, the system fails safely and requests your intervention. It will not overspend.
- **Usage Accounting:** Every action is durably logged with specific attribution (payer, auth mode, model, and rate). We never double-charge you for usage on your own API keys.
- **Evidence:** When the AI team completes a task (like creating an invoice), you receive a verified, source-linked receipt directly from the provider (e.g., Stripe), ensuring you have proof the work was completed.
