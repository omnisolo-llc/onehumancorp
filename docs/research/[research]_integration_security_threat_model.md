# Deep Dive: Security Threat Modeling for Third-Party Integrations

## Executive Summary
This document outlines the threat vectors introduced by connecting the One Human Corp (OHC) platform to 20+ external APIs. The goal is to identify potential vulnerabilities in the OAuth flows, webhook ingestion pipelines, and data storage mechanisms, and prescribe specific mitigations to ensure the security of our users' businesses.

## 1. Threat Vector: OAuth Token Exfiltration

**The Threat:** An attacker gains unauthorized read access to the OHC database (e.g., via a SQL injection vulnerability in a completely unrelated feature). If the database stores third-party OAuth Access Tokens and Refresh Tokens in plaintext, the attacker can extract these tokens. With these tokens, the attacker can impersonate thousands of small businesses on Facebook, read their private Google Calendars, or potentially manipulate their Stripe accounts.

**The Impact:** Catastrophic. Massive brand damage, regulatory fines (GDPR/CCPA), and near-total loss of user trust.

**The Mitigation (Envelope Encryption):**
- Tokens must never be stored in plaintext.
- We will employ a Key Management Service (KMS) such as AWS KMS in the Cloud environment.
- For every new OAuth connection, a unique Data Encryption Key (DEK) is generated.
- The OAuth tokens are encrypted using AES-GCM-256 with this specific DEK.
- The DEK itself is encrypted using the KMS Master Key.
- The database stores the ciphertext of the tokens and the encrypted DEK.
- When the OHC backend needs to use a token, it sends the encrypted DEK to the KMS. The KMS decrypts it and returns the plaintext DEK in memory. The backend uses this to decrypt the token, makes the API call, and then immediately zeroes out the plaintext token from memory.
- In Standalone mode, SQLCipher must be utilized to encrypt the entire SQLite database file, utilizing a key derived from the user's master password.

## 2. Threat Vector: Webhook Spoofing

**The Threat:** An attacker discovers the URL of the OHC webhook ingestion endpoint (e.g., `https://api.onehumancorp.com/webhooks/stripe`). The attacker crafts a malicious JSON payload indicating that a $10,000 invoice has been successfully paid and sends it to the endpoint.

**The Impact:** High. The OHC system marks the invoice as paid, and the business owner ships the product or provides the service, resulting in a direct financial loss to the small business owner.

**The Mitigation (Cryptographic Signature Validation):**
- All supported integration partners (Stripe, Meta, Shippo) cryptographically sign their webhook payloads using a shared secret established during the OAuth handshake.
- The webhook ingestion endpoint must strictly validate this signature *before* parsing the JSON body.
- For example, Stripe includes the `Stripe-Signature` header. The OHC backend must compute the HMAC-SHA256 of the raw request body using the webhook secret and compare it to the provided header.
- Any webhook failing signature validation must be dropped immediately with a 400 Bad Request, and a security alert should be logged.

## 3. Threat Vector: Server-Side Request Forgery (SSRF)

**The Threat:** Certain integrations might allow the user to input a URL (e.g., configuring a custom webhook destination within an advanced setting). If the OHC backend blindly makes an HTTP request to this user-provided URL, an attacker could supply a URL pointing to internal, protected infrastructure (e.g., `http://169.254.169.254/latest/meta-data/` on AWS).

**The Impact:** Critical. The attacker could extract cloud instance metadata, potentially gaining access to internal IAM roles and compromising the entire cloud infrastructure.

**The Mitigation (Strict Egress Filtering):**
- The OHC backend must employ strict URL validation, rejecting any URLs pointing to loopback addresses, private IP ranges (10.0.0.0/8, 192.168.0.0/16, etc.), or the AWS metadata endpoint.
- Furthermore, the worker nodes responsible for executing outbound HTTP requests to integrations must be placed in a private subnet with strict egress firewall rules, allowing traffic only to the documented IP ranges of the approved integration partners (e.g., allowing traffic to `api.twilio.com` but blocking all other outbound traffic).

## 4. Threat Vector: Cross-Tenant Data Leakage

**The Threat:** A bug in the application logic allows Tenant A to query the integration data or utilize the OAuth tokens belonging to Tenant B. For example, an API endpoint like `GET /api/v1/integrations/status?tenant_id=123` fails to verify if the currently authenticated user actually belongs to `tenant_id=123`.

**The Impact:** Critical. Total breach of data privacy.

**The Mitigation (Row-Level Security):**
- While application-level authorization checks are necessary, they are prone to human error.
- We must implement PostgreSQL Row-Level Security (RLS) as a defense-in-depth measure.
- The database connection pool must set a session variable containing the authenticated user's `tenant_id` at the start of every transaction.
- The RLS policies on tables like `oauth_connections` and `synced_contacts` will ensure that the database engine itself rejects any query attempting to read rows where the `tenant_id` does not match the session variable. This guarantees isolation at the lowest possible layer.

## 5. Threat Vector: Malicious Payload Injection (XSS/SQLi)

**The Threat:** An attacker sends a malicious payload via a third-party integration. For example, a customer sends an Instagram DM containing a Cross-Site Scripting (XSS) payload: `<script>alert('hacked')</script>`. The Meta Graph API correctly delivers this payload via webhook to OHC.

**The Impact:** High. If the OHC frontend blindly renders this string in the Unified Inbox, the script executes in the browser of the small business owner, potentially stealing their session cookies and compromising their OHC account.

**The Mitigation (Strict Output Encoding):**
- We must assume that all data ingested from external APIs is hostile and untrusted.
- The Data Normalization Layer must sanitize incoming strings, removing potentially dangerous characters.
- More importantly, the React frontend must employ strict output encoding. React generally handles this safely by default (escaping strings before rendering), but developers must be explicitly prohibited from using `dangerouslySetInnerHTML` when rendering any content originating from an external integration.

## Conclusion
Integrating with external APIs inherently increases the complexity of our security posture. By systematically addressing these threat vectors through Envelope Encryption, rigorous Signature Validation, strict Egress Filtering, Row-Level Security, and defensive frontend practices, we can build an integration ecosystem that is highly functional without compromising the safety of our users' businesses.
