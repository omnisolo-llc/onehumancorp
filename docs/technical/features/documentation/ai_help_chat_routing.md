# AI Help Chat Routing & RAG Context Flow

## Overview
The "Ask anything" AI-powered help chat is a floating widget accessible from every page in the OHC application. It is designed to act as a highly knowledgeable, infinitely patient first line of support for small business owners, deflecting traditional support tickets by providing instant, localized answers based strictly on official OHC Help Center documentation.

## Core Components

1. **Floating Chat Widget**: A lightweight React component injected at the root level of the application hierarchy.
2. **Specialized Help Agent**: A dedicated L4/L5 KAIROS Agent profile configured specifically for user support and documentation retrieval.
3. **RAG Pipeline**: The retrieval-augmented generation backend that connects the Help Agent to the vector database containing the MkDocs Help Center index.

## Interaction Flow

When a user opens the floating widget and types a question (e.g., "How do I refund a customer?"), the following sequence occurs:

1. **Query Interception**: The UI sends the raw user query to the `Universal Transport Bridge` (UTB) over WebSocket.
2. **Context Enrichment**: The frontend appends the user's current contextual state to the payload:
   - Current URL / Route (e.g., `/payments/dashboard`)
   - User's subscription tier
   - Language/Locale preferences
3. **Routing**: The `Omni-Context Routing Gateway` intercepts the message. Because the source is the `help_chat_widget`, it bypasses general-purpose processing and routes the payload directly to the specialized **Help Agent** pool.

## RAG Retrieval & Synthesis

Once the Help Agent receives the query, it executes the RAG flow:

1. **Embedding Generation**: The user's query is converted into a dense vector embedding using the configured embedding model (e.g., `text-embedding-3-small`).
2. **Vector Search**: The Agent queries the `VectorRepository` (pgvector in Cloud, sqlite-vec in Standalone) for the top `K=5` most relevant documentation chunks.
3. **Filtering**: The results are filtered based on the user's context (e.g., excluding API documentation if the user is not marked as an advanced user).
4. **Synthesis Prompt Generation**: The Agent constructs a prompt combining the user's query, the retrieved Markdown chunks, and the strict system persona.

### System Persona Constraints
The Help Agent is governed by strict system prompts:
- "You are a helpful assistant for non-technical small business owners."
- "You must answer the question using ONLY the provided documentation context."
- "Do not use technical jargon. Maintain an 8th-grade reading level."
- "If the documentation does not contain the answer, politely state that you do not know and offer to connect them to human support."

## Response Formatting & Deep Linking

The LLM synthesis must include structured metadata so the UI can render "Read the full article →" links.

```json
{
  "answer_text": "To refund a customer, go to your Payments tab, click on the specific transaction, and select the 'Refund' button.",
  "source_links": [
    {
      "title": "Accepting Payments and Refunds",
      "url": "/help/payments"
    }
  ]
}
```

The frontend floating widget parses this response, displaying the `answer_text` in a chat bubble and rendering the `source_links` as clickable buttons below the message, ensuring the user can always navigate to the canonical source of truth.
