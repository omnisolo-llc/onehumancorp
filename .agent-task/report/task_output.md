---
task_type: report
author: Agent
issue_id: 23289
topic: Unified Multimodal Autonomous Customer Support Engine
---

# Unified Multimodal Autonomous Customer Support Engine - Research Report

## 1. Executive Summary

Small business owners (like Maya and Carlos) lack a unified, AI-driven way to handle customer inquiries across fragmented channels (Instagram DMs, WhatsApp, SMS, Web Chat). This report introduces a comprehensive architecture design and issue brief for the Unified Multimodal Autonomous Customer Support Engine, which provides an omnichannel gateway, confidence-based AI routing (auto-reply vs. escalate), and mobile-first (375px) UI flows for the owner to review drafted responses.

## 2. Problem Statement

Business owners are overwhelmed by messages coming from multiple channels. They need:
- An omnichannel gateway to centralize messages.
- AI-driven auto-replies for common inquiries (e.g., "Do you do vegan cakes?").
- Confidence-based routing to escalate complex issues.
- A mobile-first UI (375px) to quickly review and approve AI drafts.

## 3. Architecture Design

### 3.1 Omnichannel Gateway
A centralized message ingestion layer that handles webhooks from Instagram, WhatsApp, SMS (Twilio), and Web Chat. Messages are normalized into a unified `SupportTicket` model.

### 3.2 Confidence-Based AI Routing
When a message arrives:
1.  **Ingestion:** The message is parsed and context (customer history, inventory) is retrieved.
2.  **Evaluation:** The AI Agent evaluates the message and generates a draft response.
3.  **Confidence Scoring:** The AI assigns a confidence score (0-100%).
4.  **Action:**
    *   **High Confidence (>90%):** Auto-reply directly to the customer.
    *   **Medium Confidence (50-90%):** Draft a response and escalate to the owner for review.
    *   **Low Confidence (<50%):** Flag for manual owner intervention.

### 3.3 Data Models (PostgreSQL)

```sql
CREATE TABLE support_tickets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    customer_id UUID REFERENCES customers(id),
    channel VARCHAR(50) NOT NULL, -- 'instagram', 'whatsapp', 'sms', 'web'
    external_message_id VARCHAR(255),
    status VARCHAR(50) NOT NULL, -- 'open', 'draft', 'resolved'
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE ticket_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID NOT NULL REFERENCES support_tickets(id),
    sender_type VARCHAR(50) NOT NULL, -- 'customer', 'ai', 'owner'
    content TEXT NOT NULL,
    ai_confidence DECIMAL(5,2),
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### 3.4 Mobile-First UI Flows
- **Inbox View:** A consolidated list of open tickets, clearly badged by channel.
- **Draft Review View:** A dedicated screen for the owner to quickly read the customer message, review the AI draft, and tap "Approve & Send" or "Edit".
- **Responsive Layout:** All views must fit perfectly within a 375px width (mobile-first).

## 4. Security & Isolation
- All tables must implement Row-Level Security (RLS) bound to `tenant_id`.
- External webhooks must verify signatures (e.g., Twilio signature, Meta Graph API signature).

## 5. Next Steps
- Implement the omnichannel webhook receiver endpoints in Rust.
- Develop the Confidence-Based AI Router using the primary LLM provider.
- Build the React/Tauri mobile-first Inbox and Draft Review components.
