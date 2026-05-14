# AI-Powered Help Chat Architecture

*Note: This document outlines the Retrieval-Augmented Generation (RAG) system powering the floating help chat.*

## User Experience
A floating "?" chat bubble sits in the bottom right corner of every screen. Tapping it opens a conversational interface.

The user can ask plain-language questions like "How do I give someone their money back?" The AI will respond with instructions and provide a direct link to the relevant Help Center article.

## System Architecture

### 1. Document Ingestion
Every night, a background worker scrapes the entire `docs/help_center` directory. It splits the markdown files into chunks and generates semantic embeddings using OpenAI's embedding model. These are stored in a vector database.

### 2. Query Processing
When a user asks a question:
1. The question is embedded.
2. We perform a vector search to find the top 3 most relevant documentation chunks.
3. The system prompt instructs the LLM to act as a friendly support agent for One Human Corp.

### 3. LLM Instructions (System Prompt)
```text
You are a friendly, helpful support agent for the One Human Corp Small Business App.
Your user is a non-technical small business owner.
Always answer in simple, plain English (8th-grade reading level max).
NEVER use technical jargon.

Use the following documentation context to answer the user's question:
{retrieved_context}

If the context does not contain the answer, say "I'm not exactly sure, but let me connect you with a human support rep." Do not make up answers.

Always end your response with a link to the full article using this format: "Read more here: [Article Name](url)".
```

### 4. Human Handoff
If the AI detects frustration (via sentiment analysis) or if the user explicitly asks for a human, the chat session is immediately routed to the Zendesk integration, and a real support agent takes over the thread.

## Chat Interface Components

### Quick Actions
Below the initial welcome message, the chat interface displays three "Quick Action" buttons for the most common issues:
1. "How do I refund an order?"
2. "Where is my money?"
3. "How do I add a new product?"
Tapping one of these immediately submits the question without the user needing to type.

### Typing Indicators
To make the AI feel more natural, a "Typing..." indicator (three bouncing dots) is displayed while the backend processes the vector search and generates the LLM response. This delay is artificially padded to a minimum of 800ms to avoid feeling "too robotic."

### Feedback Mechanism
Every AI response includes small "Thumbs Up" and "Thumbs Down" icons.
- If a user taps "Thumbs Down", a secondary prompt asks "What went wrong?".
- This feedback is logged to our data pipeline to help us improve the documentation and system prompts.

## Security and Privacy
- The Help Chat system does **not** have access to the user's specific financial data or customer PII. It can only answer questions about *how* to use the platform.
- If a user asks "What is John Doe's address?", the AI is instructed to reply: "I don't have access to your customers' private information, but you can find it by going to your Orders tab and searching for John Doe."
