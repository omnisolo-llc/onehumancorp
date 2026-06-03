# Unified Multimodal Autonomous Customer Support Engine Architecture

## 1. Overview
The Unified Multimodal Autonomous Customer Support Engine is designed to give small business owners a unified, AI-driven way to handle customer inquiries across multiple platforms (e.g., Instagram DMs, WhatsApp, SMS, Web Chat).

## 2. Core Components
*   **Omnichannel Gateway:** Central ingestion service to normalize messages across different channels.
*   **Confidence-based Routing:** Routing engine that evaluates AI-generated responses. If confidence > threshold, auto-reply. If confidence < threshold, create draft.
*   **Mobile-first Review UI (375px):** An interface specifically tailored for small business owners on mobile devices to easily review, edit, and approve drafted responses.
*   **Memory Integration:** Ties into the pgvector `memory` layer for historical context.

## 3. Data Flow
1. Customer sends message to a channel (e.g., Instagram).
2. Omnichannel Gateway receives webhook, standardizes format, and emits event.
3. Customer Success Agent receives event, queries `memory` for context.
4. Agent generates draft response + confidence score.
5. If score high: Omnichannel Gateway sends response back via API.
6. If score low: Draft is stored in DB.
7. Business owner uses Mobile-first UI to approve/edit.
8. Approved draft is sent via Omnichannel Gateway.
