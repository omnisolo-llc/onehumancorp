# AI-Powered Help Chat

## Overview
The "Ask anything" floating chat button appears on the bottom right of every screen in the OHC application.
When clicked, it opens a side panel directly connected to the Help Agent.

## Core Capabilities
- **Search Help Center**: The agent uses the `docs/business/help_center.md` and related Markdown content as its RAG knowledge base.
- **Provide Links**: Every response must include a "Read the full article →" link if applicable.
- **Context Awareness**: The agent knows which screen the user is currently on and will tailor its advice accordingly.
