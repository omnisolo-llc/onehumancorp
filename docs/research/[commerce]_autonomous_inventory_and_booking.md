# [commerce] Autonomous Inventory & Omni-Sync Agent

## Title
Autonomous Inventory & Omni-Sync Agent

## Problem Statement
Small business owners, like Priya (a boutique owner), struggle with managing inventory across multiple channels (in-store physical sales and online website sales). They often oversell because they forget to manually update the online website after a physical sale. They find complex e-commerce platforms (like Shopify) overwhelming because they require multiple paid apps and manual data entry just to keep basic inventory synchronized.

## Research Report
- **Findings**: Shopify and similar platforms often require third-party paid apps for seamless multi-channel inventory sync, adding complexity ("The App Tax") and cost. Users frequently complain about setup overwhelm.
- **Data**: Our deep dive into competitor capabilities revealed a significant gap. While Shopify has advanced manual inventory management, it lacks an AI-native approach that simplifies the process for non-technical users.
- **Competitive Comparison**: OHC currently lacks robust inventory management compared to Shopify, but we have a distinct advantage in our AI Swarm architecture.
- **Sources**: See the main research report in `.agent-task/report/task_output.md` for full references (50+ URLs analyzed).

## Design Doc
- **High-Level Architecture**:
  - `ProductEntity`: Core representation of a sellable item.
  - `InventoryEntity`: Tracks stock levels across channels.
  - `OmniSyncAgent`: An AI agent responsible for interpreting user inputs (images, natural language), creating product entries, and updating stock levels automatically based on sale events (online or offline).
- **UI Wireframes/Screen Flow**:
  1. **Mobile UX Flow (375px first)**:
     - The user opens the OHC app.
     - Taps "Add Item".
     - Snaps a photo of the item.
     - Types a quick natural language prompt: "5 of these dresses in M, 3 in L, $45 each."
     - The OmniSyncAgent processes this, drafts a product listing, updates the database, and presents a simple "Looks good? [Approve]" button.
  2. **Alert Flow**:
     - When an item is sold out, the user receives a simple chat notification: "The Red Summer Dress is sold out. Mark as unavailable or accept pre-orders?"
- **AI Agent Integration Points**: The OmniSyncAgent integrates directly with the chat interface to receive commands and report back. It hooks into the order processing pipeline to decrement inventory automatically upon a sale.

## Implementation Prompt
**User-Facing Outcome**:
The user can add new products and manage inventory entirely through natural language and images, without ever filling out a complex form. Inventory counts automatically update when sales occur, and the user is proactively notified of low stock via a simple chat interface.

**Critical User Journey**:
1. User snaps a photo of a new product and types a short description of stock and price.
2. The AI agent automatically creates the product listing and sets the inventory count.
3. A customer buys the product online.
4. The AI agent automatically reduces the inventory count.
5. If the count reaches zero, the AI agent asks the user via chat what to do next.

**Acceptance Criteria**:
- A new product can be created via image + text prompt.
- Inventory counts decrement correctly upon an order event.
- A low-stock or out-of-stock notification is generated and sent to the user's agent dashboard/inbox.

## Priority
P1

## Estimated Scope
Medium
