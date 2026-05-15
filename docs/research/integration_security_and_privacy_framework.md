# OHC Integration Security & Privacy Framework (Q2)

## Principles of Trust
Small business owners trust OHC with their customers, their money, and their time. Every integration must adhere to these strict standards to maintain that trust.

---

## 1. Minimal Data Sharing (The "Need to Know" Rule)
OHC should never send more data to a tool than is absolutely necessary for the task.

- **Masking**: Customer names should be masked in social tools (Buffer) unless a reply is being sent.
- **Ephemeral Storage**: Temporary data (like a shipping quote from AfterShip) must be deleted from OHC cloud once the order is complete.
- **Scoped Permissions**: We always request the "Least Privilege" scopes from Microsoft and Square.

## 2. Standalone Sovereignty
For users running OHC in **Standalone** mode, their data stays on their machine.

- **Local Secrets**: All connection tokens must be stored in the user's OS-level secure vault.
- **Direct-to-Tool**: Standalone apps should talk directly to the provider's communication link whenever possible, avoiding OHC's central servers to preserve the user's privacy.

## 3. Economic Protection (The "Born Legal" Policy)
Tool integrations must not create unexpected liabilities or costs.

- **Rate Limit Alerts**: OHC will monitor the user's usage against the tool's free tier and alert them *before* they incur overage charges.
- **Audit Logs**: Every automated post or message sent by an AI agent must be recorded in a local log for the owner to review.

## 4. Automatic Update Integrity
OHC verifies the authenticity of every automatic update from providers like Square or Buffer to ensure it hasn't been tampered with.

## 5. The "Revoke All" Kill-Switch
OHC provides a single button in Settings to "Disconnect All Tools," which immediately revokes all third-party permissions and wipes any local caches.
