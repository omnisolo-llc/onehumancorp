# [architecture]_autonomous_live_selling_and_shoppable_video_mesh.md

## Title: Autonomous Live-Selling & Shoppable Video Mesh

### Problem Statement

For OneHumanCorp’s core personas selling physical products—especially **Priya (boutique owner)** and **Maya (baker)**—social media live streams (e.g., Instagram Live, TikTok Live) are massive revenue drivers. However, managing transactions during a live stream is incredibly chaotic.

When Priya goes live to sell a limited-run collection, customers comment "Sold #123" or "Mine M". Priya has to manually write down names, cross-reference inventory in real-time, mentally track who claimed what, and then manually DM payment links or invoices after the stream ends. This results in:
1.  **Lost Sales:** Invoices aren't sent fast enough, or customers change their minds.
2.  **Overselling:** Items are claimed by multiple people, leading to awkward cancellations and poor customer experience.
3.  **High Cognitive Load:** The business owner is forced to be an entertainer, inventory manager, and billing clerk simultaneously.

Competitors like CommentSold exist but require setting up a completely separate, complex system, forcing customers out of their native social apps into clunky external checkout flows, and requiring the merchant to manually sync inventory back to their main store. Small business owners need an integrated, invisible system where an AI agent watches the live stream, handles the inventory locking, and automatically securely DMs one-click checkout links to the customers directly on the platform they are using.

### Research Report

**Competitor Analysis:**
*   **CommentSold:** The current market leader for boutique live selling.
    *   *Strengths:* Robust waitlists, auto-invoicing via Messenger.
    *   *Weaknesses:* Clunky user interface, high monthly fees, forces merchants into a distinct ecosystem separate from their primary website (e.g., Shopify), high friction for first-time buyers who must create a CommentSold account.
*   **Shopify Live / Social Integrations:**
    *   *Strengths:* Deep integration with the merchant's core catalog.
    *   *Weaknesses:* Requires buyers to leave the social app to check out on a website. Doesn't effectively handle the "comment to buy" dynamic natively without third-party apps.
*   **Instagram/TikTok Native Shopping:**
    *   *Strengths:* Frictionless checkout within the app.
    *   *Weaknesses:* Huge platform fees, strict approval processes, limited catalog syncing, and the merchant doesn't fully "own" the customer data.

**The OneHumanCorp Advantage:**
OHC will use an **AI Social Commerce Agent** combined with a **Distributed Inventory Ledger** to provide a zero-configuration, native-feeling live selling experience.

1.  **Zero Platform Switching:** The merchant goes live on Instagram/TikTok using their phone as usual.
2.  **Autonomous Observation:** The AI Agent connects via Official Graph APIs (Meta/TikTok) to "listen" to the live stream comments.
3.  **Real-Time Processing:** The AI parses natural language ("I want the blue dress in small!", "Sold 14"), instantly checks the OHC Universal Capacity and Inventory Ledger, and places a temporary "Live Lock" on the item.
4.  **Instant Conversion:** The AI Agent instantly DMs the customer a highly optimized, OHC-hosted 1-Tap Checkout link using the merchant's connected payment gateway (Stripe/Apple Pay/Google Pay).

### Design Doc

#### Architecture Diagram

```mermaid
graph TD;
    subgraph Social Media Platform (Instagram/TikTok Live)
        LiveStream[Merchant Live Stream]
        Comments[Viewer Comments]
    end

    subgraph OneHumanCorp (Multi-Tenant Hybrid Cloud)
        SocialAgent[AI Social Commerce Agent\n- Parses NL Comments\n- Matches to Catalog]
        InventoryMesh[(Universal Capacity & Inventory Ledger\n- Row-Level Tenant Isolation)]
        LiveLock[Redis / In-Memory Lock Ledger\n- Short TTLs (e.g., 10 mins)]
        CheckoutEngine[Universal 1-Tap Checkout Engine]
    end

    subgraph Customer Mobile Device
        DMs[Customer DM Inbox]
        Browser[OHC Checkout Page]
    end

    LiveStream -->|Generates| Comments;
    Comments -->|Webhook / API Polling| SocialAgent;

    SocialAgent -->|Query & Lock| InventoryMesh;
    InventoryMesh -.->|Lock Status| LiveLock;
    LiveLock -->|Success: Claimed| SocialAgent;
    LiveLock -->|Fail: Out of Stock| SocialAgent;

    SocialAgent -->|Generate Cart URL| CheckoutEngine;
    SocialAgent -->|Send DM| DMs;

    DMs -->|Tap Link| Browser;
    Browser -->|Complete Payment| CheckoutEngine;
    CheckoutEngine -->|Confirm Sale & Commit Inventory| InventoryMesh;
```

#### Key Design Decisions

1.  **Ephemeral Locking (LiveLock):** To prevent overselling without blocking regular web traffic indefinitely, items claimed in a live stream receive a temporary lock (e.g., 10 minutes) stored in a fast in-memory store (Redis). If the checkout isn't completed in that time, the lock expires, and the next person in the AI's "Waitlist Memory" gets DM'd automatically.
2.  **Multi-Tenant Webhook Processing:** Since high-velocity live streams generate massive comment volume, the `SocialAgent` ingestion layer must be horizontally scalable, strictly validating the source platform webhook and routing the payload to the specific tenant's processing queue to guarantee isolation.
3.  **Natural Language Parsing over Strict Syntax:** Instead of forcing customers to type exactly "Sold 105", the LLM-backed agent should understand variations ("I'll take the pink one", "mine size L").
4.  **Zero-Trust Identity Flow:** The checkout link DM'd to the customer contains a secure, short-lived JWT token uniquely identifying the cart and the user's social profile to prevent link sharing/hijacking.

#### Mobile UX Flow (375px First)

**Merchant (Preparation Phase - 375px):**
1.  **Dashboard Home:** Tap "Go Live" action card.
2.  **Catalog Selection:** A clean list of products appears. The merchant taps to select the items they will feature.
3.  **Overlay Generation:** The app generates a simple list of "Live Codes" (e.g., #001 for Blue Dress, #002 for Red Hat) that the merchant can reference verbally.
4.  **Start:** The app instructs the merchant to start their live stream natively in Instagram/TikTok.

**Customer (Buying Phase - 375px):**
1.  **Comment:** Customer watches the live stream and types "Sold #001".
2.  **Instant Notification:** Within 2 seconds, a push notification appears: "Priya's Boutique sent you a message."
3.  **DM Thread:** The message reads: "Hey! We saved the Blue Dress (Small) for you! Tap to checkout (Link expires in 10 mins): [Link]"
4.  **Checkout:** Tapping the link opens a half-sheet OHC checkout page overlaying the social app. Apple Pay/Google Pay is pre-loaded.
5.  **Completion:** 1-tap fingerprint/FaceID confirmation. A success animation plays. "Thanks! Heading back to the live stream..."

### Implementation Prompt

**Prompt for Implementer Agent:**

We need to implement the "Autonomous Live-Selling Mesh" for OneHumanCorp. Your goal is to build the backend ingestion, inventory locking, and checkout link generation logic for social media live streams.

**Core User Journey (CUJ):**
1. A merchant activates "Live Mode" for specific catalog items.
2. A simulated webhook payload arrives representing a customer comment during a live stream (e.g., "Sold the blue dress").
3. The system must process the natural language comment, identify the catalog item, and attempt to secure a temporary inventory lock.
4. If successful, generate a secure, short-lived checkout URL and simulate sending it back to the customer.
5. If inventory is zero, simulate placing the user on a waitlist.

**Acceptance Criteria:**
*   **Natural Language Matching:** The agent correctly maps fuzzy user comments to exact catalog variants.
*   **Concurrency Control:** Simulate 5 simultaneous "sold" comments for an item with a stock of 3. Only the first 3 must secure locks; the remaining 2 are waitlisted.
*   **Ephemeral Locks:** Implement a mechanism where a lock automatically releases after a configurable timeout (e.g., 5 seconds for testing).
*   **Multi-Tenant Isolation:** Ensure that webhook payloads are strictly validated and routed to the correct merchant's ledger context.
*   **No specific implementation details are prescribed.** You own the data schema, API endpoints, and internal module structure, provided they adhere to our high-performance, edge-caching, and zero-trust guidelines.

### Priority
`P1` (High - direct revenue enabler and strong competitive differentiator)

### Estimated Scope
Large