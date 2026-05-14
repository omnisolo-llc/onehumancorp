# OHC Documentation System Architecture

## 1. Overview
The OHC Documentation System is designed to empower non-technical small business owners. Documentation is a core product feature aimed at achieving zero support tickets for "how do I do X?".

## 2. Architecture & Data Model

### 2.1 In-App Help Center
- **Storage**: Markdown files stored in `docs/business/`.
- **Rendering**: Parsed and rendered natively within the Tauri desktop app and Next.js web client.
- **Search**: Client-side full-text search index (MiniSearch) hydrated at startup.

### 2.2 Contextual Tooltips Registry
- **Data Model**: Centralized `tooltip_registry.yaml` defining elements by ID.
- **Agent Access**: Agents can update descriptions via the Registry API without touching UI code.
- **UX**: Plain language, max 2 sentences. Hover on desktop, long-press on mobile.

### 2.3 Interactive Walkthroughs
- **Data Model**: JSON-based step sequences defining target DOM nodes, speech bubble text, and action triggers.
- **Rendering**: Overlay highlight layer, no modal popups.

### 2.4 AI-Powered Help Chat
- **Routing**: Floating "Ask anything" button routes directly to a specialized Support Agent.
- **RAG Context**: The Support Agent uses the Help Center markdown files as its exclusive vector store.
- **UX Requirement**: Must provide "Read the full article →" links.

### 2.5 Video Tutorials
- **Metadata**: Stored in PostgreSQL `video_tutorials` table (ID, Title, Duration, URL, Tags).
- **UX Requirement**: Portrait-optimized video player on mobile, <90s duration.

### 2.6 API Documentation
- **Target**: Advanced users (e.g., custom checkout integrations).
- **Rendering**: OpenAPI spec rendered via Swagger UI. Not promoted to new users.

### 2.7 Release Notes & Changelog
- **UX**: "What's New" UI overlay with plain language updates and screenshots.
- **Source**: Backed by `docs/public/release_notes.md`.
