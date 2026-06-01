issue_title: "[Security] Implement Unified Webhook Security and Replay Protection Mesh"
issue_description: |
  **Problem Statement**
  Small business owners rely on dozens of integrated systems to run their operations—from Stripe for payments, to Calendly for bookings, to Shippo for logistics. OneHumanCorp (OHC) aggregates these via webhooks to update the unified ledger, inventory, and AI agent memory. However, currently, webhook handlers lack strict cryptographic signature verification and replay protection. A malicious actor could spoof a Stripe `checkout.session.completed` event to falsely grant a tenant a "Pro" subscription or mark an order as paid. From the user's perspective, this leads to lost revenue, fraudulent access, and a breakdown of trust in the platform's reliability.

  **Research Report**
  - **Shopify:** Utilizes strict HMAC-SHA256 signature verification for all incoming webhooks. They also enforce a maximum processing window (e.g., 5 seconds) and require developers to acknowledge receipts before performing heavy background processing.
  - **Stripe:** Exposes a robust signature scheme (`Stripe-Signature` header) that includes a timestamp to prevent replay attacks.
  - **Wix/Squarespace:** Implements OAuth 2.0 and signed JWTs for all partner app webhooks.

  For OHC to serve millions of SMBs safely, we cannot implicitly trust incoming HTTP payloads. Every AI Agent department (Operations, Finance, Legal) that reacts to a webhook must be guaranteed that the payload is authentic, unmodified, and recent.

  **Key Learnings**
  1. Implicit Trust is a Vulnerability.
  2. Without timestamp validation and idempotency keys, an attacker can replay an old valid webhook.
  3. Cryptographic verification should happen at the Edge or API Gateway level before the payload ever reaches the AI Agent Queue.

  **Design Doc**
  - **Unified Signature Verification Middleware:** Implement a middleware layer that intercepts all webhook traffic. It will dynamically fetch the correct tenant/platform secret based on the route and verify the cryptographic signature.
  - **Idempotency & Replay Protection:** Store processed webhook `id`s in a high-speed cache with a 24-hour TTL. If a webhook ID is seen again, safely ignore it. Verify that the timestamp in the signature header is within 5 minutes of the current server time.
  - **Agent Handoff:** Once verified, the webhook is placed into the job queue. The API immediately returns `200 OK` to the provider, preventing timeouts, while the AI Agent processes the business logic asynchronously.
  - **Zero Trust Isolation:** Each tenant has isolated webhook secrets encrypted via SPIFFE/SPIRE identity standards.

  *Architecture diagram (Mermaid.js):*
  ```mermaid
  sequenceDiagram
      participant Provider as External Provider
      participant API as OHC API Gateway
      participant Cache as High Speed Cache
      participant Queue as Job Queue
      participant Agent as AI Agent

      Provider->>API: POST /webhook (Payload + Signature)
      API->>API: Extract Signature & Timestamp
      API->>API: Verify Cryptographic Signature
      alt Invalid Signature or Expired Timestamp
          API-->>Provider: 401 Unauthorized
      else Valid Signature
          API->>Cache: Check Idempotency Key (Event ID)
          alt Already Processed
              API-->>Provider: 200 OK (Duplicate ignored)
          else New Event
              API->>Cache: Store Event ID (TTL 24h)
              API->>Queue: Insert into AI Job Queue
              API-->>Provider: 200 OK (Accepted)
              Queue-->>Agent: Dequeue and Execute Workflow
          end
      end
  ```
  *UI wireframes / Mobile UX flow:* N/A - Backend Infrastructure only. No visual UI changes.
  *AI agent integration points:* Validates payloads before ingestion into the Operations, Finance, and Legal agent memory queues.

  **Implementation Prompt**
  **Task for Implementer Agent:**
  Implement the Zero-Trust Webhook Security Mesh.
  1. Create a middleware layer that enforces cryptographic signature validation using provider-specific secrets.
  2. Update all existing webhook handlers to utilize this middleware.
  3. Add a cache-backed replay protection check that ensures no event ID is processed twice.
  4. Ensure the API immediately acknowledges the webhook after enqueueing the payload to prevent provider timeouts.
  5. Provide 100% unit test coverage simulating valid, invalid, and replayed webhook signatures. Do not prescribe specific library or framework choices; design the tests to validate the behavioral invariants.

  **Estimated Scope:** Medium

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
