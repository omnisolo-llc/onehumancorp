# Help Center & Documentation Architecture

## Problem Statement
Small business owners (our primary demographic) often find traditional software documentation overly technical and difficult to discover. OHC needs a documentation architecture that treats help content as a core product feature rather than an afterthought, achieving the "zero support tickets" goal.

## Architecture Components

1. **In-App Help Center**: A searchable help portal accessible from every major screen. Content is built using Markdown and managed alongside the codebase.
2. **Contextual Tooltips Registry**: A decoupled registry containing plain language tooltips for UI elements. Designed for programmatic consumption without tying tooltip text to React components.
3. **Interactive Walkthroughs**: A state-machine-driven guide overlay that highlights UI elements and provides contextual speech bubbles without using modal dialogs that obscure the screen.
4. **AI-Powered Help Chat**: A floating assistant that uses a specialized RAG pipeline over the help center content. It intercepts user queries and provides localized answers with direct links back to full articles.

## Content Structure and Guidelines
- **Audience**: Small business owners.
- **Reading Level**: Maximum 8th-grade reading level.
- **Tone**: Plain language, zero technical jargon.

## Data Flow for Documentation
1. Help articles are authored in Markdown under `docs/business/public/app/help_center/`.
2. MkDocs generates the searchable site payload.
3. The RAG pipeline indexes the generated output for the AI Support Agent.
4. The Tooltip Registry exposes a JSON/Proto schema containing localized UI help text.

## RAG Indexing Details

To ensure the AI Support Agent can effectively assist small business owners, all help center documentation is indexed into our vector search backend using the HybridCache architecture.
This means:
- The `VectorRepository` memory layer maintains parity across Cloud (PostgreSQL pgvector) and Standalone (SQLite sqlite-vec) modes.
- Documentation embeddings are recalculated automatically upon PR merge via the CI/CD pipeline, ensuring the AI agent always references the most up-to-date instructions.

## UI Components & Integration

### The Help Button
A globally accessible "?" floating action button is present across all core feature screens (Store, Marketing, AI Agents). Clicking it opens an overlay with:
1. A search bar directly wired to the Help Center MkDocs index.
2. The AI Help Chat interface.
3. Quick links to top-viewed articles.

### Tooltip Mechanics
- **Desktop**: Triggered via `onMouseEnter` with a 300ms delay to prevent visual noise.
- **Mobile**: Triggered via `onLongPress` (500ms).
- Both interaction patterns retrieve content from the global Tooltip Registry, ensuring consistency.

## Metrics & Observability

To measure the success of the documentation architecture:
1. **Search Deflection Rate**: Percentage of Help Center searches that do not result in a fallback to human support or failed task completion.
2. **Tooltip Engagement**: Frequency of tooltip activations on advanced UI elements.
3. **Walkthrough Completion Rate**: The percentage of users who complete multi-step tutorials (e.g., "Set up your store") versus those who dismiss them early.

## Cross-Mode Architecture Requirements

As with all core OHC features, the Help Center and its associated tools must operate flawlessly in both Cloud and Standalone environments.

### Cloud Native Implementation
- Documentation assets are served via the CDN with edge-caching for sub-50ms latency.
- AI Help Chat leverages the primary OHC LLM routing gateway for inference.
- Walkthrough state and analytics are synced to the central telemetry cluster.

### Standalone (Hybrid) Implementation
- Help documentation is bundled directly into the Tauri desktop application.
- `MkDocs` static assets are served over the `tauri://` custom protocol.
- The AI Help Chat utilizes the locally-hosted small language model (if available) or falls back to basic keyword search against a local SQLite index.
- Walkthrough state is maintained in the local SQLite database and synced to the cloud during the next connectivity window using the Universal Transport Bridge.

## Future Expansion

Future iterations of the documentation architecture will focus on:
- **Auto-Translation**: Real-time translation of Markdown help articles into 20+ languages based on the user's locale settings.
- **Dynamic Content Generation**: Using KAIROS to auto-generate video tutorials based on changes to the UI codebase.
- **Context-Aware Recommendations**: The AI Agent proactively suggesting help articles based on the user's current actions and historical behavior.

## Advanced Analytics and Reporting

Beyond basic engagement metrics, we implement advanced analytics to track the performance and ROI of our documentation efforts.
This includes:
- **Time-to-Resolution (TTR)**: Tracking how quickly users find answers in the Help Center compared to contacting human support.
- **Content Gap Analysis**: Identifying search terms that yield no results, automatically flagging them for new documentation creation.
- **Feedback Loops**: Integrating simple "Was this helpful?" buttons at the bottom of each article and tracking the sentiment over time.
- **Behavioral Pathing**: Analyzing the typical journey users take through the Help Center (e.g., from Getting Started -> My Store -> Payments) to optimize content flow and suggestions.
- **A/B Testing**: Randomly presenting different versions of an article or tooltip to measure which performs better in terms of clarity and helpfulness.
- **Session Replay**: For users who consent, securely recording their Help Center sessions to identify areas of confusion or friction in the UI itself.
